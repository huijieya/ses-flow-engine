pub mod app_routes;
pub mod device_routes;
pub mod flow_routes;
pub mod node_routes;
pub mod station_routes;
pub mod stats_routes;
pub mod rcs_routes;
pub mod pda_routes;
pub mod wave_routes;

use axum::{
    extract::State,
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;

use crate::core::AppState;

/// Create all API routes
pub fn create_routes() -> Router<Arc<AppState>> {
    Router::new()
        // Flow orchestration routes
        .nest("/flows", flow_routes::routes())
        .nest("/nodes", node_routes::routes())
        .nest("/apps", app_routes::routes())
        .nest("/devices", device_routes::routes())
        .nest("/stations", station_routes::routes())
        // Wave and order management
        .nest("/waves", wave_routes::routes())
        // SES 1.0 compatible stats routes (mounted at root)
        .merge(stats_routes::routes())
        // SES 1.0 external integration routes
        .merge(rcs_routes::routes())
        .merge(pda_routes::routes())
}

/// Device callback handler
pub async fn callback_handler(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    tracing::info!("Device callback: {:?}", payload);
    Json(json!({
        "success": true,
        "message": "Callback received"
    }))
}
