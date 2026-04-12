use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::core::state::AppState;
use crate::core::types::PageRequest;
use crate::core::error::Result;
use crate::models::app::{CreateAppRequest, AppResponse, UpdateAppRequest};

pub fn routes(state: Arc<RwLock<AppState>>) -> Router<Arc<RwLock<AppState>>> {
    Router::new()
        .route("/", get(list_apps).post(create_app))
        .route("/:id", get(get_app).put(update_app).delete(delete_app))
        .with_state(state)
}

async fn list_apps(
    State(_state): State<Arc<RwLock<AppState>>>,
    Query(_params): Query<PageRequest>,
) -> Result<Json<Vec<AppResponse>>> {
    Ok(Json(vec![]))
}

async fn create_app(
    State(_state): State<Arc<RwLock<AppState>>>,
    Json(_request): Json<CreateAppRequest>,
) -> Result<Json<AppResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn get_app(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<AppResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn update_app(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(_id): Path<Uuid>,
    Json(_request): Json<UpdateAppRequest>,
) -> Result<Json<AppResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn delete_app(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}
