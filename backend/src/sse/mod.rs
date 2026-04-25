//! Server-Sent Events (SSE) module
//! SSE实时推送模块 - 用于工作站和本系统的实时通信

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
    fn payload_json(&self) -> serde_json::Value {
        match self {
            SseEventType::AgvArrived { station_id, agv_id } => json!({
                "messageType": "Agv_Arrived",
                "requestId": uuid::Uuid::new_v4().to_string(),
                "data": {
                    "StationId": station_id,
                    "AgvId": agv_id
                },
                "timestamp": chrono::Utc::now().timestamp_millis()
            }),
            SseEventType::AgvLeft { station_id, agv_id } => json!({
                "messageType": "AGV_DEPART",
                "requestId": uuid::Uuid::new_v4().to_string(),
                "data": {
                    "StationId": station_id,
                    "AgvId": agv_id
                },
                "timestamp": chrono::Utc::now().timestamp_millis()
            }),
            SseEventType::TaskAssigned { station_id, task } => json!({
                "messageType": "Task_Assigned",
                "data": {
                    "StationId": station_id,
                    "Task": task
                },
                "timestamp": chrono::Utc::now().timestamp_millis()
            }),
            SseEventType::ChuteStatusChanged { grid_id, status } => json!({
                "messageType": "Chute_Status_Changed",
                "data": {
                    "GridId": grid_id,
                    "Status": status
                },
                "timestamp": chrono::Utc::now().timestamp_millis()
            }),
            SseEventType::WallStatusChanged { wall_id, status } => json!({
                "messageType": "Wall_Status_Changed",
                "data": {
                    "WallId": wall_id,
                    "Status": status
                },
                "timestamp": chrono::Utc::now().timestamp_millis()
            }),
            SseEventType::WaveStarted { wave_id } => json!({
                "messageType": "WAVE_START",
                "data": {
                    "WaveId": wave_id
                },
                "timestamp": chrono::Utc::now().timestamp_millis()
            }),
            SseEventType::WaveClosed { wave_id } => json!({
                "messageType": "WAVE_CLOSE",
                "data": {
                    "WaveId": wave_id
                },
                "timestamp": chrono::Utc::now().timestamp_millis()
            }),
            SseEventType::Heartbeat => json!({
                "messageType": "Heart_Beat",
                "data": {
                    "RcsStatus": "ONLINE"
                },
                "timestamp": chrono::Utc::now().timestamp_millis()
            }),
        }
    }

    pub fn payload_string(&self) -> String {
        self.payload_json().to_string()
    }

    pub fn to_sse_event(&self) -> Event {
        Event::default()
            .data(self.payload_string())
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
