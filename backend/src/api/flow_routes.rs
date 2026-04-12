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
use crate::models::flow::{
    CreateFlowRequest, ExecuteFlowRequest, FlowResponse, FlowInstanceResponse,
    UpdateFlowRequest,
};

pub fn routes(state: Arc<RwLock<AppState>>) -> Router<Arc<RwLock<AppState>>> {
    Router::new()
        .route("/", get(list_flows).post(create_flow))
        .route("/:id", get(get_flow).put(update_flow).delete(delete_flow))
        .route("/:id/execute", post(execute_flow))
        .route("/:id/instances", get(list_instances))
        .route("/:instance_id/resume", post(resume_flow))
        .route("/:instance_id/cancel", post(cancel_flow))
        .with_state(state)
}

async fn list_flows(
    State(_state): State<Arc<RwLock<AppState>>>,
    Query(_params): Query<ListFlowsParams>,
) -> Result<Json<Vec<FlowResponse>>> {
    // Placeholder implementation
    Ok(Json(vec![]))
}

#[derive(Debug, Deserialize)]
struct ListFlowsParams {
    app_id: Option<Uuid>,
    page: Option<i64>,
    size: Option<i64>,
}

async fn create_flow(
    State(_state): State<Arc<RwLock<AppState>>>,
    Json(_request): Json<CreateFlowRequest>,
) -> Result<Json<FlowResponse>> {
    // Placeholder implementation
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn get_flow(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<FlowResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn update_flow(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(_id): Path<Uuid>,
    Json(_request): Json<UpdateFlowRequest>,
) -> Result<Json<FlowResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn delete_flow(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn execute_flow(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(_id): Path<Uuid>,
    Json(_request): Json<ExecuteFlowRequest>,
) -> Result<Json<FlowInstanceResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn list_instances(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(_id): Path<Uuid>,
    Query(_params): Query<PageRequest>,
) -> Result<Json<Vec<FlowInstanceResponse>>> {
    Ok(Json(vec![]))
}

async fn resume_flow(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(_instance_id): Path<Uuid>,
) -> Result<Json<FlowInstanceResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn cancel_flow(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(_instance_id): Path<Uuid>,
) -> Result<Json<FlowInstanceResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}
