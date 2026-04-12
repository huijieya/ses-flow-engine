pub mod app;
pub mod device;
pub mod flow;
pub mod node;
pub mod task;
pub mod wave;
pub mod order;
pub mod station;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Base model trait for common fields
pub trait BaseModel {
    fn id(&self) -> Uuid;
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
}

/// Common response DTO with ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdResponse {
    pub id: Uuid,
}
