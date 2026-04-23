pub mod json_utils;
pub mod validation;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub fn generate_id() -> Uuid {
    Uuid::new_v4()
}

pub fn now() -> DateTime<Utc> {
    Utc::now()
}
