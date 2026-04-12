use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Device model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Device {
    pub id: Uuid,
    pub app_id: Uuid,
    pub device_id: String,
    pub device_name: String,
    pub device_type: DeviceType,
    pub ip: Option<String>,
    pub port: Option<i32>,
    pub online: bool,
    pub status: DeviceStatus,
    pub config: Option<serde_json::Value>,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "device_type", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum DeviceType {
    Hub,
    Button,
    Line,
    Lift,
    Sorter,
    Agv,
    Printer,
    Scanner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "device_status", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum DeviceStatus {
    Idle,
    Busy,
    Error,
    Offline,
    Maintenance,
}

/// Chute model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Chute {
    pub id: Uuid,
    pub app_id: Uuid,
    pub chute_id: String,
    pub platform_id: String,
    pub chute_type: ChuteType,
    pub status: ChuteStatus,
    pub physical_status: Option<serde_json::Value>,
    pub io_status: Option<serde_json::Value>,
    pub light_status: Option<serde_json::Value>,
    pub device_id: Option<String>,
    pub node_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "chute_type", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ChuteType {
    Normal,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "chute_status", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ChuteStatus {
    Open,
    Close,
    Full,
    Disable,
    Error,
}

/// Create device request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeviceRequest {
    pub device_id: String,
    pub device_name: String,
    pub device_type: DeviceType,
    pub ip: Option<String>,
    pub port: Option<i32>,
    pub config: Option<serde_json::Value>,
}

/// Update device request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeviceRequest {
    pub device_name: Option<String>,
    pub ip: Option<String>,
    pub port: Option<i32>,
    pub config: Option<serde_json::Value>,
    pub status: Option<DeviceStatus>,
}

/// Create chute request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChuteRequest {
    pub chute_id: String,
    pub platform_id: String,
    pub chute_type: ChuteType,
    pub device_id: Option<String>,
    pub node_id: Option<i32>,
}

/// Update chute request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChuteRequest {
    pub status: Option<ChuteStatus>,
    pub physical_status: Option<serde_json::Value>,
    pub io_status: Option<serde_json::Value>,
    pub light_status: Option<serde_json::Value>,
}

/// Device task model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DeviceTask {
    pub id: Uuid,
    pub device_id: String,
    pub app_id: Uuid,
    pub task_type: String,
    pub command_data: serde_json::Value,
    pub status: TaskStatus,
    pub result: Option<serde_json::Value>,
    pub retry_count: i32,
    pub max_retries: i32,
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
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Device response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceResponse {
    pub id: Uuid,
    pub device_id: String,
    pub device_name: String,
    pub device_type: DeviceType,
    pub online: bool,
    pub status: DeviceStatus,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<Device> for DeviceResponse {
    fn from(device: Device) -> Self {
        Self {
            id: device.id,
            device_id: device.device_id,
            device_name: device.device_name,
            device_type: device.device_type,
            online: device.online,
            status: device.status,
            last_heartbeat: device.last_heartbeat,
            created_at: device.created_at,
        }
    }
}

/// Chute response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChuteResponse {
    pub id: Uuid,
    pub chute_id: String,
    pub platform_id: String,
    pub chute_type: ChuteType,
    pub status: ChuteStatus,
    pub device_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Chute> for ChuteResponse {
    fn from(chute: Chute) -> Self {
        Self {
            id: chute.id,
            chute_id: chute.chute_id,
            platform_id: chute.platform_id,
            chute_type: chute.chute_type,
            status: chute.status,
            device_id: chute.device_id,
            created_at: chute.created_at,
        }
    }
}
