use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Wave model representing a batch of orders
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Wave {
    pub id: Uuid,
    pub wave_id: String,
    pub app_id: Uuid,
    pub wave_name: String,
    pub priority: i32,
    pub status: String,
    pub platform_id: Option<String>,
    pub total_orders: i32,
    pub completed_orders: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum WaveStatus {
    Created,
    Started,
    Paused,
    Closed,
    Cancelled,
}

impl WaveStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            WaveStatus::Created => "CREATED",
            WaveStatus::Started => "STARTED",
            WaveStatus::Paused => "PAUSED",
            WaveStatus::Closed => "CLOSED",
            WaveStatus::Cancelled => "CANCELLED",
        }
    }
}

impl std::fmt::Display for WaveStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Create wave request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWaveRequest {
    pub wave_id: String,
    pub wave_name: String,
    pub wave_type: Option<String>,
    pub priority: Option<i32>,
    pub platform_id: Option<String>,
}

/// Update wave request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWaveRequest {
    pub wave_name: Option<String>,
    pub priority: Option<i32>,
    pub status: Option<String>,
}

/// Wave response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveResponse {
    pub id: Uuid,
    pub wave_id: String,
    pub wave_name: String,
    pub priority: i32,
    pub status: String,
    pub platform_id: Option<String>,
    pub total_orders: i32,
    pub completed_orders: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<Wave> for WaveResponse {
    fn from(wave: Wave) -> Self {
        Self {
            id: wave.id,
            wave_id: wave.wave_id,
            wave_name: wave.wave_name,
            priority: wave.priority,
            status: wave.status,
            platform_id: wave.platform_id,
            total_orders: wave.total_orders,
            completed_orders: wave.completed_orders,
            started_at: wave.started_at,
            completed_at: wave.completed_at,
            created_at: wave.created_at,
        }
    }
}
