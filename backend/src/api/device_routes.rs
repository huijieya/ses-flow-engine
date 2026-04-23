use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::core::state::AppState;
use crate::core::types::PageRequest;
use crate::core::error::Result;
use crate::models::device::{
    CreateDeviceRequest, CreateChuteRequest, DeviceResponse, ChuteResponse,
    UpdateDeviceRequest, UpdateChuteRequest,
};
use crate::models::task::{CreateTaskRequest, TaskCallbackRequest, TaskResponse};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_devices).post(create_device))
        .route("/:id", get(get_device).put(update_device).delete(delete_device))
        .route("/:id/status", get(get_device_status))
        .route("/:id/task", post(send_task))
        .route("/chutes", get(list_chutes).post(create_chute))
        .route("/chutes/:id", get(get_chute).put(update_chute))
        .route("/callback", post(handle_callback))
}

async fn list_devices(
    State(_state): State<Arc<AppState>>,
    Query(_params): Query<ListDevicesParams>,
) -> Result<Json<Vec<DeviceResponse>>> {
    Ok(Json(vec![]))
}

#[derive(Debug, Deserialize)]
struct ListDevicesParams {
    app_id: Option<Uuid>,
    device_type: Option<String>,
    online: Option<bool>,
}

async fn create_device(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<CreateDeviceRequest>,
) -> Result<Json<DeviceResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn get_device(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<DeviceResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn update_device(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<Uuid>,
    Json(_request): Json<UpdateDeviceRequest>,
) -> Result<Json<DeviceResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn delete_device(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn get_device_status(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn send_task(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<Uuid>,
    Json(_request): Json<CreateTaskRequest>,
) -> Result<Json<TaskResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn list_chutes(
    State(_state): State<Arc<AppState>>,
    Query(_params): Query<ListChutesParams>,
) -> Result<Json<Vec<ChuteResponse>>> {
    Ok(Json(vec![]))
}

#[derive(Debug, Deserialize)]
struct ListChutesParams {
    app_id: Option<Uuid>,
    platform_id: Option<String>,
    status: Option<String>,
}

async fn create_chute(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<CreateChuteRequest>,
) -> Result<Json<ChuteResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn get_chute(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<ChuteResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn update_chute(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<Uuid>,
    Json(_request): Json<UpdateChuteRequest>,
) -> Result<Json<ChuteResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn handle_callback(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<TaskCallbackRequest>,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"success": true})))
}
