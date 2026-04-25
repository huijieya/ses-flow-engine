use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Workstation model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Station {
    pub id: Uuid,
    pub station_id: String,
    pub app_id: Uuid,
    pub platform_id: String,
    pub station_name: String,
    pub station_type: StationType,
    pub status: StationStatus,
    pub operator_id: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub config: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "station_type", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum StationType {
    Induction,
    Packing,
    Sorting,
    Returns,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "station_status", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum StationStatus {
    Offline,
    Online,
    Busy,
    Paused,
}

/// Station login request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationLoginRequest {
    #[serde(rename = "StationId", alias = "stationId", alias = "station_id")]
    pub station_id: String,
    #[serde(rename = "PlatformId", alias = "platformId", alias = "platform_id")]
    pub platform_id: String,
    #[serde(rename = "Username", alias = "username")]
    pub username: String,
    #[serde(rename = "Password", alias = "password")]
    pub password: String,
}

/// Station operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationOperationRequest {
    pub station_id: String,
    pub platform_id: String,
}

/// Station task request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationTaskRequest {
    #[serde(rename = "StationId", alias = "stationId", alias = "station_id")]
    pub station_id: String,
    #[serde(rename = "PlatformId", alias = "platformId", alias = "platform_id")]
    pub platform_id: String,
    #[serde(rename = "Sku", alias = "sku")]
    pub sku: Option<String>,
    #[serde(rename = "Barcode", alias = "barcode")]
    pub barcode: Option<String>,
    #[serde(rename = "Completed", alias = "completed")]
    pub completed: Option<i32>,
    #[serde(rename = "WaveType", alias = "waveType", alias = "wave_type")]
    pub wave_type: Option<String>,
    #[serde(rename = "LockId", alias = "lockId", alias = "lock_id")]
    pub lock_id: Option<String>,
}

/// Station scan request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationScanRequest {
    #[serde(default, rename = "StationId", alias = "stationId", alias = "station_id")]
    pub station_id: String,
    #[serde(default, rename = "PlatformId", alias = "platformId", alias = "platform_id")]
    pub platform_id: String,
    #[serde(rename = "Barcode", alias = "barcode")]
    pub barcode: String,
}

/// Station dispatch request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationDispatchRequest {
    pub station_id: String,
    pub platform_id: String,
    pub task_id: String,
    pub sku: String,
    pub barcode: String,
    pub wave_id: String,
    pub order_id: String,
}

/// Station robot departure request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationDepartRequest {
    #[serde(default, rename = "StationId", alias = "stationId", alias = "station_id")]
    pub station_id: String,
    #[serde(default, rename = "PlatformId", alias = "platformId", alias = "platform_id")]
    pub platform_id: String,
    #[serde(rename = "TaskId", alias = "taskId", alias = "task_id")]
    pub task_id: String,
    #[serde(rename = "AgvId", alias = "agvId", alias = "agv_id")]
    pub agv_id: String,
    #[serde(rename = "Completed", alias = "completed")]
    pub completed: i32,
    #[serde(rename = "RequestId", alias = "requestId", alias = "request_id")]
    pub request_id: Option<String>,
}

/// Station response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationResponse {
    pub id: Uuid,
    pub station_id: String,
    pub platform_id: String,
    pub station_name: String,
    pub station_type: StationType,
    pub status: StationStatus,
    pub operator_id: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<Station> for StationResponse {
    fn from(station: Station) -> Self {
        Self {
            id: station.id,
            station_id: station.station_id,
            platform_id: station.platform_id,
            station_name: station.station_name,
            station_type: station.station_type,
            status: station.status,
            operator_id: station.operator_id,
            last_login_at: station.last_login_at,
            created_at: station.created_at,
        }
    }
}

/// Login response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationLoginResponse {
    #[serde(rename = "Code")]
    pub code: i32,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Data")]
    pub data: Option<LoginOutputDto>,
}

/// Task info response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfoResponse {
    #[serde(rename = "Code")]
    pub code: i32,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Data")]
    pub data: Option<TaskInfoData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginOutputDto {
    #[serde(rename = "Authorization")]
    pub authorization: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyNotifyRequest {
    #[serde(rename = "SseRequestId", alias = "sseRequestId", alias = "requestId")]
    pub sse_request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationItemInfoVo {
    #[serde(rename = "Sku")]
    pub sku: String,
    #[serde(rename = "SkuName")]
    pub sku_name: String,
    #[serde(rename = "Barcode")]
    pub barcode: String,
    #[serde(rename = "ImageUrl")]
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfoData {
    #[serde(rename = "TaskId")]
    pub task_id: String,
    #[serde(rename = "ChuteId")]
    pub chute_id: String,
    #[serde(rename = "WaveId")]
    pub wave_id: String,
    #[serde(rename = "OrderId")]
    pub order_id: String,
    #[serde(rename = "Count")]
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectRequestDto {
    #[serde(rename = "ClientId", alias = "clientId")]
    pub client_id: String,
    #[serde(rename = "PlatformId", alias = "platformId")]
    pub platform_id: String,
    #[serde(rename = "StationIds", alias = "stationIds")]
    pub station_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseResult<T> {
    #[serde(rename = "Code")]
    pub code: i32,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Data")]
    pub data: Option<T>,
}
