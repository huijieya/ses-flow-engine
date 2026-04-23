use serde::{Deserialize, Serialize};

/// Standard device status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatusResponse {
    pub device_id: String,
    pub online: bool,
    pub status: String,
    pub chutes: Option<Vec<ChuteStatus>>,
    pub battery_level: Option<i32>,
    pub position: Option<Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChuteStatus {
    pub chute_id: String,
    pub status: String,
    pub full: bool,
    pub open: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

/// Task request to device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequest {
    pub task_id: String,
    pub task_type: String,
    pub priority: i32,
    pub payload: serde_json::Value,
}

/// Task response from device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResponse {
    pub task_id: String,
    pub accepted: bool,
    pub estimated_duration_ms: Option<u64>,
    pub error: Option<String>,
}

/// Callback message from device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCallback {
    pub task_id: String,
    pub device_id: String,
    pub status: String,
    pub result: Option<serde_json::Value>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub completed_at: String,
}
