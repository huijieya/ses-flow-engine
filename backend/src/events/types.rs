use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Event type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    // Flow events
    FlowStarted,
    FlowCompleted,
    FlowFailed,
    FlowCancelled,

    // Node events
    NodeStarted,
    NodeCompleted,
    NodeFailed,
    NodeRetrying,

    // Device events
    DeviceConnected,
    DeviceDisconnected,
    DeviceStatusChanged,
    TaskCompleted,
    TaskFailed,

    // Workstation events
    StationLogin,
    StationLogout,
    StationOnline,
    StationOffline,
    BarcodeScanned,

    // Wave events
    WaveCreated,
    WaveStarted,
    WaveClosed,

    // Order events
    OrderCreated,
    OrderAssigned,
    OrderCompleted,

    // Custom events
    Custom(String),
}

impl EventType {
    pub fn as_str(&self) -> String {
        match self {
            EventType::FlowStarted => "FLOW_STARTED".to_string(),
            EventType::FlowCompleted => "FLOW_COMPLETED".to_string(),
            EventType::FlowFailed => "FLOW_FAILED".to_string(),
            EventType::FlowCancelled => "FLOW_CANCELLED".to_string(),
            EventType::NodeStarted => "NODE_STARTED".to_string(),
            EventType::NodeCompleted => "NODE_COMPLETED".to_string(),
            EventType::NodeFailed => "NODE_FAILED".to_string(),
            EventType::NodeRetrying => "NODE_RETRYING".to_string(),
            EventType::DeviceConnected => "DEVICE_CONNECTED".to_string(),
            EventType::DeviceDisconnected => "DEVICE_DISCONNECTED".to_string(),
            EventType::DeviceStatusChanged => "DEVICE_STATUS_CHANGED".to_string(),
            EventType::TaskCompleted => "TASK_COMPLETED".to_string(),
            EventType::TaskFailed => "TASK_FAILED".to_string(),
            EventType::StationLogin => "STATION_LOGIN".to_string(),
            EventType::StationLogout => "STATION_LOGOUT".to_string(),
            EventType::StationOnline => "STATION_ONLINE".to_string(),
            EventType::StationOffline => "STATION_OFFLINE".to_string(),
            EventType::BarcodeScanned => "BARCODE_SCANNED".to_string(),
            EventType::WaveCreated => "WAVE_CREATED".to_string(),
            EventType::WaveStarted => "WAVE_STARTED".to_string(),
            EventType::WaveClosed => "WAVE_CLOSED".to_string(),
            EventType::OrderCreated => "ORDER_CREATED".to_string(),
            EventType::OrderAssigned => "ORDER_ASSIGNED".to_string(),
            EventType::OrderCompleted => "ORDER_COMPLETED".to_string(),
            EventType::Custom(s) => s.clone(),
        }
    }
}

/// Event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub event_type: EventType,
    pub app_id: Uuid,
    pub source: String,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub correlation_id: Option<Uuid>,
    pub context: HashMap<String, String>,
}

impl Event {
    pub fn new(
        event_type: EventType,
        app_id: Uuid,
        source: impl Into<String>,
        payload: impl Serialize,
    ) -> Result<Self, serde_json::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            event_type,
            app_id,
            source: source.into(),
            payload: serde_json::to_value(payload)?,
            timestamp: Utc::now(),
            correlation_id: None,
            context: HashMap::new(),
        })
    }

    pub fn with_correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }
}

/// Event handler trait
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    /// Returns true if this handler can handle the given event type
    fn can_handle(&self, event_type: &EventType) -> bool;

    /// Handle an event
    async fn handle(&self, event: &Event) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Event subscription
#[derive(Debug, Clone)]
pub struct EventSubscription {
    pub id: Uuid,
    pub event_types: Vec<EventType>,
    pub handler: String,
    pub filter: Option<EventFilter>,
}

/// Event filter for subscriptions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFilter {
    pub source_pattern: Option<String>,
    pub payload_filter: Option<serde_json::Value>,
}

impl EventFilter {
    pub fn matches(&self, event: &Event) -> bool {
        if let Some(pattern) = &self.source_pattern {
            if !event.source.contains(pattern) {
                return false;
            }
        }

        if let Some(filter) = &self.payload_filter {
            return Self::json_matches(filter, &event.payload);
        }

        true
    }

    fn json_matches(filter: &serde_json::Value, payload: &serde_json::Value) -> bool {
        match (filter, payload) {
            (serde_json::Value::Object(filter_map), serde_json::Value::Object(payload_map)) => {
                filter_map.iter().all(|(key, filter_value)| {
                    payload_map
                        .get(key)
                        .map_or(false, |payload_value| Self::json_matches(filter_value, payload_value))
                })
            }
            (a, b) => a == b,
        }
    }
}

/// Task completed event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCompletedPayload {
    pub task_id: Uuid,
    pub device_id: String,
    pub result: serde_json::Value,
    pub duration_ms: u64,
}

/// Barcode scanned event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeScannedPayload {
    pub station_id: String,
    pub barcode: String,
    pub sku: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Device status changed event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatusChangedPayload {
    pub device_id: String,
    pub old_status: String,
    pub new_status: String,
    pub reason: Option<String>,
}
