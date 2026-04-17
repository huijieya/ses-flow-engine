//! Server-Sent Events (SSE) module
//! SSE实时推送模块 - 用于工作站和RCS的实时通信

use axum::{
    response::sse::{Event, Sse},
    extract::Path,
    extract::State,
};
use futures::stream::{self, Stream};
use std::collections::HashMap;
use std::sync::Arc;
use std::pin::Pin;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, error, info, warn};
use serde_json::json;

pub mod manager;

pub use manager::SseManager;

/// SSE event types
#[derive(Debug, Clone)]
pub enum SseEventType {
    /// AGV arrived at station
    AgvArrived { station_id: String, agv_id: String },
    /// AGV left station
    AgvLeft { station_id: String, agv_id: String },
    /// Task assigned to station
    TaskAssigned { station_id: String, task: serde_json::Value },
    /// Chute status changed
    ChuteStatusChanged { grid_id: String, status: String },
    /// Wall status changed
    WallStatusChanged { wall_id: String, status: String },
    /// Wave started
    WaveStarted { wave_id: String },
    /// Wave closed
    WaveClosed { wave_id: String },
    /// Heartbeat
    Heartbeat,
}

impl SseEventType {
    pub fn to_sse_event(&self) -> Event {
        let (event_name, data) = match self {
            SseEventType::AgvArrived { station_id, agv_id } => (
                "agv_arrived",
                json!({
                    "station_id": station_id,
                    "agv_id": agv_id,
                    "timestamp": chrono::Utc::now().timestamp_millis()
                }),
            ),
            SseEventType::AgvLeft { station_id, agv_id } => (
                "agv_left",
                json!({
                    "station_id": station_id,
                    "agv_id": agv_id,
                    "timestamp": chrono::Utc::now().timestamp_millis()
                }),
            ),
            SseEventType::TaskAssigned { station_id, task } => (
                "task_assigned",
                json!({
                    "station_id": station_id,
                    "task": task,
                    "timestamp": chrono::Utc::now().timestamp_millis()
                }),
            ),
            SseEventType::ChuteStatusChanged { grid_id, status } => (
                "chute_status_changed",
                json!({
                    "grid_id": grid_id,
                    "status": status,
                    "timestamp": chrono::Utc::now().timestamp_millis()
                }),
            ),
            SseEventType::WallStatusChanged { wall_id, status } => (
                "wall_status_changed",
                json!({
                    "wall_id": wall_id,
                    "status": status,
                    "timestamp": chrono::Utc::now().timestamp_millis()
                }),
            ),
            SseEventType::WaveStarted { wave_id } => (
                "wave_started",
                json!({
                    "wave_id": wave_id,
                    "timestamp": chrono::Utc::now().timestamp_millis()
                }),
            ),
            SseEventType::WaveClosed { wave_id } => (
                "wave_closed",
                json!({
                    "wave_id": wave_id,
                    "timestamp": chrono::Utc::now().timestamp_millis()
                }),
            ),
            SseEventType::Heartbeat => (
                "heartbeat",
                json!({"timestamp": chrono::Utc::now().timestamp_millis()}),
            ),
        };

        Event::default()
            .event(event_name)
            .data(data.to_string())
    }
}

/// SSE event for broadcasting
#[derive(Debug, Clone)]
pub struct SseBroadcastEvent {
    pub target: SseTarget,
    pub event: SseEventType,
}

/// Target for SSE event
#[derive(Debug, Clone)]
pub enum SseTarget {
    /// Specific station
    Station(String),
    /// Specific platform
    Platform(String),
    /// All RCS clients
    AllRcs,
    /// All stations
    AllStations,
    /// Broadcast to all
    Broadcast,
}

/// Station connection info
#[derive(Debug)]
pub struct StationConnection {
    pub station_id: String,
    pub platform_id: String,
    pub sender: broadcast::Sender<SseEventType>,
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

/// RCS SSE connection for receiving grid status updates
#[derive(Debug)]
pub struct RcsSseConnection {
    pub rcs_id: String,
    pub platform_ids: Vec<String>,
    pub sender: broadcast::Sender<SseEventType>,
}
