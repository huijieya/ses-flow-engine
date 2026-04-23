use crate::core::error::{Result, SesError};
use crate::core::types::{ExecutionContext, JsonValue, NodeKind};
use crate::engine::executor::NodeExecutor;
use crate::models::node::{NodeConfig, NodeExecutionResult};
use async_trait::async_trait;
use std::sync::Arc;

/// Condition router node
pub struct ConditionRouterNode;

#[async_trait]
impl NodeExecutor for ConditionRouterNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Logic
    }

    fn node_type(&self) -> &str {
        "condition_router"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let condition = config.params.get("condition")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("condition is required".to_string()))?;

        let true_branch = config.params.get("trueBranch")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let false_branch = config.params.get("falseBranch")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Evaluate condition (simplified - in production use proper expression evaluator)
        let result = self.evaluate_condition(condition, &input)?;

        let next_nodes = if result {
            if !true_branch.is_empty() { vec![true_branch.to_string()] } else { vec![] }
        } else {
            if !false_branch.is_empty() { vec![false_branch.to_string()] } else { vec![] }
        };

        Ok(NodeExecutionResult {
            success: true,
            output: serde_json::json!({"condition_result": result}),
            next_nodes,
            error: None,
        })
    }
}

impl ConditionRouterNode {
    fn evaluate_condition(&self, condition: &str, input: &JsonValue) -> Result<bool> {
        // Simplified condition evaluation
        // In production, use a proper expression evaluator like rhai or evalexpr
        
        // Handle simple equality: "field == value"
        if condition.contains("==") {
            let parts: Vec<&str> = condition.split("==").collect();
            if parts.len() == 2 {
                let field = parts[0].trim();
                let value = parts[1].trim().trim_matches('"');
                
                if let Some(input_value) = input.get(field) {
                    return Ok(input_value.as_str() == Some(value) || 
                              input_value.to_string().trim_matches('"') == value);
                }
            }
        }
        
        // Default to true
        Ok(true)
    }
}

/// Order type router node
pub struct OrderTypeRouterNode;

#[async_trait]
impl NodeExecutor for OrderTypeRouterNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Logic
    }

    fn node_type(&self) -> &str {
        "order_type_router"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let order_type = input.get("orderType")
            .and_then(|v| v.as_str())
            .unwrap_or("CHUTE");

        let chute_branch = config.params.get("chuteBranch")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let wall_branch = config.params.get("wallBranch")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let next_nodes = match order_type.to_uppercase().as_str() {
            "CHUTE" if !chute_branch.is_empty() => vec![chute_branch.to_string()],
            "WALL" if !wall_branch.is_empty() => vec![wall_branch.to_string()],
            _ => vec![],
        };

        Ok(NodeExecutionResult {
            success: true,
            output: serde_json::json!({"orderType": order_type}),
            next_nodes,
            error: None,
        })
    }
}

/// ForEach loop node
pub struct ForEachNode;

#[async_trait]
impl NodeExecutor for ForEachNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Logic
    }

    fn node_type(&self) -> &str {
        "foreach"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let collection = input.get("collection")
            .and_then(|v| v.as_array())
            .ok_or_else(|| SesError::Validation("collection is required".to_string()))?;

        let output = serde_json::json!({
            "success": true,
            "itemCount": collection.len(),
            "items": collection
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// Fork-Join node for parallel execution
pub struct ForkJoinNode;

#[async_trait]
impl NodeExecutor for ForkJoinNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Logic
    }

    fn node_type(&self) -> &str {
        "fork_join"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let branches = config.params.get("branches")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect::<Vec<String>>()
            })
            .unwrap_or_default();

        Ok(NodeExecutionResult {
            success: true,
            output: serde_json::json!({"branches": branches}),
            next_nodes: branches,
            error: None,
        })
    }
}

/// Wait for event node
pub struct WaitForEventNode;

#[async_trait]
impl NodeExecutor for WaitForEventNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Logic
    }

    fn node_type(&self) -> &str {
        "wait_for_event"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let event_name = config.params.get("eventName")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("eventName is required".to_string()))?;

        let timeout = config.params.get("timeout")
            .and_then(|v| v.as_u64())
            .unwrap_or(30000); // 30 seconds default

        let output = serde_json::json!({
            "success": true,
            "eventName": event_name,
            "timeout": timeout,
            "status": "waiting"
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// Register all logic nodes
pub fn register_nodes(runtime: &mut crate::engine::executor::ExecutionRuntime) {
    runtime.register(Arc::new(ConditionRouterNode));
    runtime.register(Arc::new(OrderTypeRouterNode));
    runtime.register(Arc::new(ForEachNode));
    runtime.register(Arc::new(ForkJoinNode));
    runtime.register(Arc::new(WaitForEventNode));
}
