pub mod client;
pub mod protocol;
pub mod registry;

use crate::core::error::Result;
use crate::models::device::{Device, DeviceStatus, TaskStatus};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Device capability trait
#[async_trait]
pub trait DeviceCapability: Send + Sync {
    fn device_type(&self) -> &str;
    async fn execute(&self, device: &Device, command: DeviceCommand) -> Result<DeviceResponse>;
    async fn query_status(&self, device: &Device) -> Result<DeviceStatus>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCommand {
    pub command_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCallbackMessage {
    pub device_id: String,
    pub task_id: String,
    pub status: TaskStatus,
    pub result: Option<serde_json::Value>,
    pub error_message: Option<String>,
}
