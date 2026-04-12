use crate::core::types::{ExecutionContext, ExecutionStatus, Id, JsonValue};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Flow definition model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Flow {
    pub id: Uuid,
    pub app_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub flow_json: FlowDefinition,
    pub version: i32,
    pub is_template: bool,
    pub status: FlowStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "flow_status", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum FlowStatus {
    Draft,
    Published,
    Archived,
}

/// Flow definition structure (stored as JSONB)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FlowDefinition {
    pub nodes: Vec<FlowNode>,
    pub edges: Vec<FlowEdge>,
    pub variables: Option<HashMap<String, JsonValue>>,
}

/// Node in a flow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowNode {
    pub id: String,
    pub node_def_id: String,
    pub name: String,
    pub kind: String,
    pub position: NodePosition,
    pub config: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePosition {
    pub x: f64,
    pub y: f64,
}

/// Edge connecting nodes in a flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub condition: Option<String>,
}

/// Flow instance (execution record)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FlowInstance {
    pub id: Uuid,
    pub flow_id: Uuid,
    pub app_id: Uuid,
    pub status: ExecutionStatus,
    pub context: Option<ExecutionContext>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create flow request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFlowRequest {
    pub name: String,
    pub description: Option<String>,
    pub flow_json: FlowDefinition,
    pub is_template: Option<bool>,
}

/// Update flow request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFlowRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub flow_json: Option<FlowDefinition>,
    pub status: Option<FlowStatus>,
}

/// Execute flow request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteFlowRequest {
    pub initial_context: Option<ExecutionContext>,
    pub trigger_event: Option<String>,
}

/// Flow response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowResponse {
    pub id: Uuid,
    pub app_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub flow_json: FlowDefinition,
    pub version: i32,
    pub is_template: bool,
    pub status: FlowStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Flow> for FlowResponse {
    fn from(flow: Flow) -> Self {
        Self {
            id: flow.id,
            app_id: flow.app_id,
            name: flow.name,
            description: flow.description,
            flow_json: flow.flow_json,
            version: flow.version,
            is_template: flow.is_template,
            status: flow.status,
            created_at: flow.created_at,
            updated_at: flow.updated_at,
        }
    }
}

/// Flow instance response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowInstanceResponse {
    pub id: Uuid,
    pub flow_id: Uuid,
    pub status: ExecutionStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<FlowInstance> for FlowInstanceResponse {
    fn from(instance: FlowInstance) -> Self {
        Self {
            id: instance.id,
            flow_id: instance.flow_id,
            status: instance.status,
            started_at: instance.started_at,
            completed_at: instance.completed_at,
            error_message: instance.error_message,
            created_at: instance.created_at,
        }
    }
}
