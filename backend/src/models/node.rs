use crate::core::types::{ExecutionStatus, JsonValue, NodeKind, RetryPolicy};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Node definition model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NodeDefinition {
    pub id: Uuid,
    pub node_id: String,
    pub name: String,
    pub description: Option<String>,
    pub kind: NodeKind,
    pub device_type: Option<String>,
    pub input_schema: Option<JsonValue>,
    pub output_schema: Option<JsonValue>,
    pub config_schema: Option<JsonValue>,
    pub default_config: Option<JsonValue>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub category: String,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Node instance (execution record within a flow instance)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NodeInstance {
    pub id: Uuid,
    pub flow_instance_id: Uuid,
    pub node_def_id: String,
    pub node_id: String,
    pub node_name: String,
    pub node_type: String,
    pub input_data: Option<JsonValue>,
    pub output_data: Option<JsonValue>,
    pub status: ExecutionStatus,
    pub retry_count: i32,
    pub error_message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create node definition request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNodeDefinitionRequest {
    pub node_id: String,
    pub name: String,
    pub description: Option<String>,
    pub kind: NodeKind,
    pub device_type: Option<String>,
    pub input_schema: Option<JsonValue>,
    pub output_schema: Option<JsonValue>,
    pub config_schema: Option<JsonValue>,
    pub default_config: Option<JsonValue>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub category: String,
}

/// Update node definition request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNodeDefinitionRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub input_schema: Option<JsonValue>,
    pub output_schema: Option<JsonValue>,
    pub config_schema: Option<JsonValue>,
    pub default_config: Option<JsonValue>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub category: Option<String>,
}

/// Node definition response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDefinitionResponse {
    pub id: Uuid,
    pub node_id: String,
    pub name: String,
    pub description: Option<String>,
    pub kind: NodeKind,
    pub device_type: Option<String>,
    pub input_schema: Option<JsonValue>,
    pub output_schema: Option<JsonValue>,
    pub config_schema: Option<JsonValue>,
    pub default_config: Option<JsonValue>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub category: String,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NodeDefinition> for NodeDefinitionResponse {
    fn from(def: NodeDefinition) -> Self {
        Self {
            id: def.id,
            node_id: def.node_id,
            name: def.name,
            description: def.description,
            kind: def.kind,
            device_type: def.device_type,
            input_schema: def.input_schema,
            output_schema: def.output_schema,
            config_schema: def.config_schema,
            default_config: def.default_config,
            icon: def.icon,
            color: def.color,
            category: def.category,
            is_system: def.is_system,
            created_at: def.created_at,
            updated_at: def.updated_at,
        }
    }
}

/// Node instance response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInstanceResponse {
    pub id: Uuid,
    pub flow_instance_id: Uuid,
    pub node_def_id: String,
    pub node_id: String,
    pub node_name: String,
    pub node_type: String,
    pub status: ExecutionStatus,
    pub retry_count: i32,
    pub error_message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<NodeInstance> for NodeInstanceResponse {
    fn from(instance: NodeInstance) -> Self {
        Self {
            id: instance.id,
            flow_instance_id: instance.flow_instance_id,
            node_def_id: instance.node_def_id,
            node_id: instance.node_id,
            node_name: instance.node_name,
            node_type: instance.node_type,
            status: instance.status,
            retry_count: instance.retry_count,
            error_message: instance.error_message,
            started_at: instance.started_at,
            completed_at: instance.completed_at,
            created_at: instance.created_at,
        }
    }
}

/// Node execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeExecutionResult {
    pub success: bool,
    pub output: JsonValue,
    pub next_nodes: Vec<String>,
    pub error: Option<String>,
}

/// Node configuration in flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    #[serde(flatten)]
    pub params: JsonValue,
    pub retry_policy: Option<RetryPolicy>,
    pub timeout_ms: Option<u64>,
}
