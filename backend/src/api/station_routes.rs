use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::core::state::AppState;
use crate::core::types::PageRequest;
use crate::core::error::Result;
use crate::models::station::{
    StationResponse, StationLoginRequest, StationLoginResponse,
    StationOperationRequest, StationTaskRequest, StationScanRequest,
    StationDispatchRequest, StationDepartRequest, TaskInfoResponse,
};

pub fn routes(state: Arc<RwLock<AppState>>) -> Router<Arc<RwLock<AppState>>> {
    Router::new()
        .route("/", get(list_stations))
        .route("/login", post(station_login))
        .route("/logout", post(station_logout))
        .route("/online", post(station_online))
        .route("/offline", post(station_offline))
        .route("/scan", post(station_scan))
        .route("/task", post(get_task))
        .route("/distribute", post(dispatch_task))
        .route("/depart", post(robot_depart))
        .with_state(state)
}

async fn list_stations(
    State(_state): State<Arc<RwLock<AppState>>>,
    Query(_params): Query<PageRequest>,
) -> Result<Json<Vec<StationResponse>>> {
    Ok(Json(vec![]))
}

async fn station_login(
    State(_state): State<Arc<RwLock<AppState>>>,
    Json(_request): Json<StationLoginRequest>,
) -> Result<Json<StationLoginResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn station_logout(
    State(_state): State<Arc<RwLock<AppState>>>,
    Json(_request): Json<StationOperationRequest>,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"success": true})))
}

async fn station_online(
    State(_state): State<Arc<RwLock<AppState>>>,
    Json(_request): Json<StationOperationRequest>,
) -> Result<Json<StationResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn station_offline(
    State(_state): State<Arc<RwLock<AppState>>>,
    Json(_request): Json<StationOperationRequest>,
) -> Result<Json<StationResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn station_scan(
    State(_state): State<Arc<RwLock<AppState>>>,
    Json(_request): Json<StationScanRequest>,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"success": true, "sku": "SKU001", "barcode": "123456"})))
}

async fn get_task(
    State(_state): State<Arc<RwLock<AppState>>>,
    Json(_request): Json<StationTaskRequest>,
) -> Result<Json<TaskInfoResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn dispatch_task(
    State(_state): State<Arc<RwLock<AppState>>>,
    Json(_request): Json<StationDispatchRequest>,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"success": true})))
}

async fn robot_depart(
    State(_state): State<Arc<RwLock<AppState>>>,
    Json(_request): Json<StationDepartRequest>,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"success": true, "departResult": "dispatched"})))
}
