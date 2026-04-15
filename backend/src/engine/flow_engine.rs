use crate::core::error::{Result, SesError};
use crate::core::types::{ExecutionContext, ExecutionStatus, JsonValue};
use crate::engine::dag::{Dag, NodeId};
use crate::engine::executor::ExecutionRuntime;
use crate::events::EventBus;
use crate::models::flow::{Flow, FlowInstance, FlowRow};
use crate::models::node::{NodeConfig, NodeInstance};
use chrono::Utc;
use dashmap::DashMap;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};
use uuid::Uuid;

/// Flow execution engine
pub struct FlowEngine {
    db_pool: PgPool,
    event_bus: EventBus,
    runtime: ExecutionRuntime,
    /// Active flow instances
    instances: DashMap<Uuid, Arc<RwLock<FlowInstanceState>>>,
}

/// Internal state for a flow instance during execution
#[derive(Debug)]
pub struct FlowInstanceState {
    pub instance: FlowInstance,
    pub dag: Dag,
    pub node_configs: HashMap<NodeId, NodeConfig>,
    pub completed_nodes: HashSet<NodeId>,
    pub failed_nodes: HashSet<NodeId>,
    pub context: ExecutionContext,
}

impl FlowEngine {
    pub fn new(db_pool: PgPool, event_bus: EventBus) -> Self {
        let mut runtime = ExecutionRuntime::new();
        runtime.register_defaults();

        Self {
            db_pool,
            event_bus,
            runtime,
            instances: DashMap::new(),
        }
    }

    /// Start a new flow instance
    pub async fn start_flow(
        &self,
        flow_id: Uuid,
        app_id: Uuid,
        initial_context: Option<ExecutionContext>,
    ) -> Result<FlowInstance> {
        info!("Starting flow instance for flow_id: {}", flow_id);

        // Load flow definition
        let flow = self.get_flow(flow_id).await?;

        // Create instance record
        let instance_id = Uuid::new_v4();
        let instance = FlowInstance {
            id: instance_id,
            flow_id,
            app_id,
            status: ExecutionStatus::Running,
            context: initial_context.clone(),
            started_at: Some(Utc::now()),
            completed_at: None,
            error_message: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Persist instance
        self.save_instance(&instance).await?;

        // Build DAG from flow definition
        let flow_def = flow.flow_definition().map_err(|e| {
            SesError::FlowExecution(format!("Failed to parse flow definition: {}", e))
        })?;
        let (dag, node_configs) = self.build_dag(&flow_def)?;

        // Create instance state
        let state = FlowInstanceState {
            instance: instance.clone(),
            dag,
            node_configs,
            completed_nodes: HashSet::new(),
            failed_nodes: HashSet::new(),
            context: initial_context.unwrap_or_default(),
        };

        self.instances.insert(instance_id, Arc::new(RwLock::new(state)));

        // Start execution
        let engine = Arc::new(self.clone_engine());
        let instance_id_clone = instance_id;
        tokio::spawn(async move {
            if let Err(e) = engine.execute_flow_instance(instance_id_clone).await {
                error!("Flow instance {} execution failed: {}", instance_id_clone, e);
            }
        });

        Ok(instance)
    }

    /// Resume a paused flow instance
    pub async fn resume_flow(&self, instance_id: Uuid) -> Result<FlowInstance> {
        info!("Resuming flow instance: {}", instance_id);

        let state_arc = self.instances.get(&instance_id).ok_or_else(|| {
            SesError::NotFound(format!("Flow instance not found: {}", instance_id))
        })?;

        let mut state = state_arc.write().await;
        state.instance.status = ExecutionStatus::Running;
        state.instance.updated_at = Utc::now();

        drop(state);

        // Continue execution
        let engine = Arc::new(self.clone_engine());
        tokio::spawn(async move {
            if let Err(e) = engine.execute_flow_instance(instance_id).await {
                error!("Flow instance {} execution failed: {}", instance_id, e);
            }
        });

        let state = state_arc.read().await;
        let result = state.instance.clone();
        drop(state);

        Ok(result)
    }

    /// Cancel a flow instance
    pub async fn cancel_flow(&self, instance_id: Uuid) -> Result<FlowInstance> {
        info!("Cancelling flow instance: {}", instance_id);

        let state_arc = self.instances.get(&instance_id).ok_or_else(|| {
            SesError::NotFound(format!("Flow instance not found: {}", instance_id))
        })?;

        let mut state = state_arc.write().await;
        state.instance.status = ExecutionStatus::Cancelled;
        state.instance.completed_at = Some(Utc::now());
        state.instance.updated_at = Utc::now();

        self.save_instance(&state.instance).await?;

        // Clean up
        drop(state);
        self.instances.remove(&instance_id);

        let state = state_arc.read().await;
        let result = state.instance.clone();
        drop(state);

        Ok(result)
    }

    /// Execute a flow instance
    async fn execute_flow_instance(&self, instance_id: Uuid) -> Result<()> {
        let state_arc = self.instances.get(&instance_id).unwrap().clone();

        // Get root nodes
        let root_nodes = {
            let state = state_arc.read().await;
            state.dag.root_nodes()
        };

        if root_nodes.is_empty() {
            return Err(SesError::FlowExecution("No root nodes found in DAG".to_string()));
        }

        // Execute starting from root nodes
        for node_id in root_nodes {
            Box::pin(self.execute_node(instance_id, node_id)).await?;
        }

        Ok(())
    }

    /// Execute a single node
    async fn execute_node(&self, instance_id: Uuid, node_id: NodeId) -> Result<()> {
        let state_arc = self.instances.get(&instance_id).unwrap().clone();

        // Get node info
        let (node_config, node_type) = {
            let state = state_arc.read().await;
            let config = state.node_configs.get(&node_id).cloned().ok_or_else(|| {
                SesError::NodeExecution(format!("Node config not found: {}", node_id))
            })?;
            // Get node type from node_def_id in config
            let node_type = config.params.get("nodeType")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            (config, node_type)
        };

        // Create node instance record
        let node_instance_id = Uuid::new_v4();
        let node_instance = NodeInstance {
            id: node_instance_id,
            flow_instance_id: instance_id,
            node_def_id: node_id.clone(),
            node_id: node_id.clone(),
            node_name: node_id.clone(),
            node_type: node_type.clone(),
            input_data: None,
            output_data: None,
            status: ExecutionStatus::Running,
            retry_count: 0,
            error_message: None,
            started_at: Some(Utc::now()),
            completed_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.save_node_instance(&node_instance).await?;

        // Prepare input
        let input = {
            let state = state_arc.read().await;
            serde_json::to_value(&state.context).unwrap_or_default()
        };

        // Execute node
        let result = {
            let mut state = state_arc.write().await;
            self.runtime.execute(&node_type, input, &mut state.context, &node_config).await
        };

        // Handle result
        match result {
            Ok(exec_result) => {
                let mut state = state_arc.write().await;
                state.completed_nodes.insert(node_id.clone());

                // Update node instance
                let mut updated_instance = node_instance.clone();
                updated_instance.status = ExecutionStatus::Completed;
                updated_instance.output_data = Some(exec_result.output.clone());
                updated_instance.completed_at = Some(Utc::now());
                updated_instance.updated_at = Utc::now();
                self.save_node_instance(&updated_instance).await?;

                // Update context with output
                state.context.set(&format!("node_{}_output", node_id), exec_result.output);

                // Execute next nodes
                if let Some(next_nodes) = state.dag.outgoing(&node_id) {
                    let next_nodes: Vec<String> = next_nodes.clone();
                    drop(state);

                    for next_node_id in next_nodes {
                        // Check if all prerequisites are completed
                        if self.are_prerequisites_completed(instance_id, &next_node_id).await? {
                            Box::pin(self.execute_node(instance_id, next_node_id)).await?;
                        }
                    }
                } else {
                    drop(state);
                }

                // Check if flow is complete
                self.check_flow_completion(instance_id).await?;
            }
            Err(e) => {
                let mut state = state_arc.write().await;
                state.failed_nodes.insert(node_id.clone());

                // Update node instance
                let mut updated_instance = node_instance.clone();
                updated_instance.status = ExecutionStatus::Failed;
                updated_instance.error_message = Some(e.to_string());
                updated_instance.completed_at = Some(Utc::now());
                updated_instance.updated_at = Utc::now();
                self.save_node_instance(&updated_instance).await?;

                // Mark flow as failed
                state.instance.status = ExecutionStatus::Failed;
                state.instance.error_message = Some(format!("Node {} failed: {}", node_id, e));
                state.instance.completed_at = Some(Utc::now());
                state.instance.updated_at = Utc::now();
                self.save_instance(&state.instance).await?;
            }
        }

        Ok(())
    }

    /// Check if all prerequisites for a node are completed
    async fn are_prerequisites_completed(&self, instance_id: Uuid, node_id: &NodeId) -> Result<bool> {
        let state_arc = self.instances.get(&instance_id).unwrap();
        let state = state_arc.read().await;

        if let Some(incoming) = state.dag.incoming(node_id) {
            for prereq in incoming {
                if !state.completed_nodes.contains(prereq) {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// Check if flow execution is complete
    async fn check_flow_completion(&self, instance_id: Uuid) -> Result<()> {
        let state_arc = self.instances.get(&instance_id).unwrap();
        let state = state_arc.read().await;

        let total_nodes = state.dag.nodes().len();
        let completed_nodes = state.completed_nodes.len();

        if completed_nodes == total_nodes {
            info!("Flow instance {} completed successfully", instance_id);

            let mut updated_instance = state.instance.clone();
            drop(state);

            updated_instance.status = ExecutionStatus::Completed;
            updated_instance.completed_at = Some(Utc::now());
            updated_instance.updated_at = Utc::now();

            self.save_instance(&updated_instance).await?;
            self.instances.remove(&instance_id);
        }

        Ok(())
    }

    /// Build DAG from flow definition
    fn build_dag(&self, flow_def: &crate::models::flow::FlowDefinition) -> Result<(Dag, HashMap<NodeId, NodeConfig>)> {
        let mut dag = Dag::new();
        let mut node_configs = HashMap::new();

        // Add nodes
        for node in &flow_def.nodes {
            dag.add_node(node.id.clone())?;

            let config = NodeConfig {
                params: node.config.clone(),
                retry_policy: None,
                timeout_ms: None,
            };
            node_configs.insert(node.id.clone(), config);
        }

        // Add edges
        for edge in &flow_def.edges {
            dag.add_edge(edge.source.clone(), edge.target.clone(), edge.condition.clone())?;
        }

        Ok((dag, node_configs))
    }

    /// Get flow by ID
    async fn get_flow(&self, flow_id: Uuid) -> Result<Flow> {
        let row: FlowRow = sqlx::query_as::<_, FlowRow>(
            r#"
            SELECT id, app_id, name, description, flow_json, version, is_template, status, created_at, updated_at, created_by
            FROM ses_flows
            WHERE id = $1
            "#
        )
        .bind(flow_id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(row.into())
    }

    /// Save flow instance
    async fn save_instance(&self, instance: &FlowInstance) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO ses_flow_instances (id, flow_id, app_id, status, context, started_at, completed_at, error_message, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                context = EXCLUDED.context,
                started_at = EXCLUDED.started_at,
                completed_at = EXCLUDED.completed_at,
                error_message = EXCLUDED.error_message,
                updated_at = EXCLUDED.updated_at
            "#
        )
        .bind(instance.id)
        .bind(instance.flow_id)
        .bind(instance.app_id)
        .bind(instance.status)
        .bind(instance.context.as_ref().map(|c| serde_json::to_value(c).unwrap()))
        .bind(instance.started_at)
        .bind(instance.completed_at)
        .bind(&instance.error_message)
        .bind(instance.created_at)
        .bind(instance.updated_at)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    /// Save node instance
    async fn save_node_instance(&self, instance: &NodeInstance) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO ses_node_instances (
                id, flow_instance_id, node_def_id, node_id, node_name, node_type,
                input_data, output_data, status, retry_count, error_message,
                started_at, completed_at, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                input_data = EXCLUDED.input_data,
                output_data = EXCLUDED.output_data,
                retry_count = EXCLUDED.retry_count,
                error_message = EXCLUDED.error_message,
                completed_at = EXCLUDED.completed_at,
                updated_at = EXCLUDED.updated_at
            "#
        )
        .bind(instance.id)
        .bind(instance.flow_instance_id)
        .bind(&instance.node_def_id)
        .bind(&instance.node_id)
        .bind(&instance.node_name)
        .bind(&instance.node_type)
        .bind(&instance.input_data)
        .bind(&instance.output_data)
        .bind(instance.status)
        .bind(instance.retry_count)
        .bind(&instance.error_message)
        .bind(instance.started_at)
        .bind(instance.completed_at)
        .bind(instance.created_at)
        .bind(instance.updated_at)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    /// Clone the engine (for spawning tasks)
    fn clone_engine(&self) -> Self {
        Self {
            db_pool: self.db_pool.clone(),
            event_bus: self.event_bus.clone(),
            runtime: ExecutionRuntime::new(),
            instances: self.instances.clone(),
        }
    }

    /// Execute a single node directly without a full flow instance.
    /// Used by SES 1.0 compatible API endpoints to route through orchestration nodes.
    pub async fn execute_node_direct(
        &self,
        node_type: &str,
        input: JsonValue,
        context: &mut ExecutionContext,
    ) -> Result<crate::models::node::NodeExecutionResult> {
        let config = NodeConfig {
            params: serde_json::json!({}),
            retry_policy: None,
            timeout_ms: None,
        };
        self.runtime.execute(node_type, input, context, &config).await
    }
}

impl Clone for FlowEngine {
    fn clone(&self) -> Self {
        self.clone_engine()
    }
}
