use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::core::state::AppState;
use crate::core::types::PageRequest;
use crate::core::error::Result;
use crate::models::node::{
    CreateNodeDefinitionRequest, NodeDefinitionResponse, UpdateNodeDefinitionRequest,
};

pub fn routes(state: Arc<RwLock<AppState>>) -> Router<Arc<RwLock<AppState>>> {
    Router::new()
        .route("/definitions", get(list_definitions).post(create_definition))
        .route("/definitions/:id", get(get_definition).put(update_definition).delete(delete_definition))
        .with_state(state)
}

async fn list_definitions(
    State(_state): State<Arc<RwLock<AppState>>>,
    Query(_params): Query<ListDefinitionsParams>,
) -> Result<Json<Vec<NodeDefinitionResponse>>> {
    Ok(Json(vec![]))
}

#[derive(Debug, Deserialize)]
struct ListDefinitionsParams {
    kind: Option<String>,
    category: Option<String>,
    page: Option<i64>,
    size: Option<i64>,
}

async fn create_definition(
    State(_state): State<Arc<RwLock<AppState>>>,
    Json(_request): Json<CreateNodeDefinitionRequest>,
) -> Result<Json<NodeDefinitionResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn get_definition(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<NodeDefinitionResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn update_definition(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(_id): Path<Uuid>,
    Json(_request): Json<UpdateNodeDefinitionRequest>,
) -> Result<Json<NodeDefinitionResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn delete_definition(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}
