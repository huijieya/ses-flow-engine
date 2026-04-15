use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Generic ID type alias
pub type Id = Uuid;

/// JSON value for flexible data storage
pub type JsonValue = serde_json::Value;

/// Context data passed between nodes during execution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionContext {
    #[serde(flatten)]
    pub data: HashMap<String, JsonValue>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.data
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    pub fn set<T: serde::Serialize>(&mut self, key: &str, value: T) {
        if let Ok(json_value) = serde_json::to_value(value) {
            self.data.insert(key.to_string(), json_value);
        }
    }

    pub fn merge(&mut self, other: ExecutionContext) {
        self.data.extend(other.data);
    }
}

/// Execution status for flows and nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "execution_status", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Retry,
    Cancelled,
    Paused,
}

impl Default for ExecutionStatus {
    fn default() -> Self {
        ExecutionStatus::Pending
    }
}

/// Node kind classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "node_kind", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum NodeKind {
    Device,
    Logic,
    Data,
    Query,
    System,
    Task,
}

/// Device type for device nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DeviceType {
    Chute,
    Sorter,
    Agv,
    Printer,
    Scanner,
    Wall,
}

/// Retry policy for node execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_ms: u64,
    pub max_backoff_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            backoff_ms: 500,
            max_backoff_ms: 30000,
        }
    }
}

/// Page request for pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    pub page: i64,
    pub size: i64,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self { page: 1, size: 20 }
    }
}

/// Page response for pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub size: i64,
    pub pages: i64,
}

impl<T> PageResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i64, size: i64) -> Self {
        let pages = (total as f64 / size as f64).ceil() as i64;
        Self {
            data,
            total,
            page,
            size,
            pages,
        }
    }
}

/// API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(code: u16, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code,
                message: message.into(),
            }),
        }
    }
}

/// Common API list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
}

/// Timestamp wrapper with created/updated fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamped {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Timestamped {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
        }
    }
}
