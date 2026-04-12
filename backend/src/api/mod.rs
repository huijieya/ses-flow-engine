pub mod flow_routes;
pub mod node_routes;
pub mod device_routes;
pub mod app_routes;
pub mod station_routes;

use axum::{extract::State, response::Response, Json};
use crate::core::state::AppState;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SharedState = State<Arc<RwLock<AppState>>>;

pub async fn callback_handler() -> &'static str {
    "Callback received"
}
