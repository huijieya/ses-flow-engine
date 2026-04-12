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
    pub current_user: Option<String>,
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
    pub station_id: String,
    pub platform_id: String,
    pub username: String,
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
    pub station_id: String,
    pub platform_id: String,
    pub sku: Option<String>,
    pub barcode: Option<String>,
}

/// Station scan request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationScanRequest {
    pub station_id: String,
    pub platform_id: String,
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
    pub station_id: String,
    pub platform_id: String,
    pub task_id: String,
    pub agv_id: String,
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
    pub current_user: Option<String>,
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
            current_user: station.current_user,
            last_login_at: station.last_login_at,
            created_at: station.created_at,
        }
    }
}

/// Login response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationLoginResponse {
    pub token: String,
    pub station: StationResponse,
}

/// Task info response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfoResponse {
    pub task_id: String,
    pub order_id: String,
    pub wave_id: String,
    pub destination: String,
    pub sku: String,
    pub qty: i32,
}
