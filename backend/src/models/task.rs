use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Task model representing a job dispatched to devices
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: Uuid,
    pub app_id: Uuid,
    pub flow_instance_id: Option<Uuid>,
    pub node_instance_id: Option<Uuid>,
    pub device_id: String,
    pub task_type: String,
    pub priority: i32,
    pub payload: serde_json::Value,
    pub status: TaskStatus,
    pub result: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum TaskStatus {
    Pending,
    Dispatched,
    Running,
    Completed,
    Failed,
    Cancelled,
    Timeout,
}

/// Create task request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub device_id: String,
    pub task_type: String,
    pub priority: Option<i32>,
    pub payload: serde_json::Value,
    pub flow_instance_id: Option<Uuid>,
    pub node_instance_id: Option<Uuid>,
}

/// Task callback request from device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCallbackRequest {
    pub task_id: Uuid,
    pub device_id: String,
    pub status: TaskStatus,
    pub result: Option<serde_json::Value>,
    pub error_message: Option<String>,
}

/// Task response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResponse {
    pub id: Uuid,
    pub device_id: String,
    pub task_type: String,
    pub priority: i32,
    pub status: TaskStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            device_id: task.device_id,
            task_type: task.task_type,
            priority: task.priority,
            status: task.status,
            started_at: task.started_at,
            completed_at: task.completed_at,
            created_at: task.created_at,
        }
    }
}
