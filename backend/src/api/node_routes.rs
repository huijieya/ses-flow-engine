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
    NodeDefinition,
};

pub fn routes(state: Arc<RwLock<AppState>>) -> Router<Arc<RwLock<AppState>>> {
    Router::new()
        .route("/definitions", get(list_definitions).post(create_definition))
        .route("/definitions/:id", get(get_definition).put(update_definition).delete(delete_definition))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
struct ListDefinitionsParams {
    kind: Option<String>,
    category: Option<String>,
    page: Option<i64>,
    size: Option<i64>,
}

async fn list_definitions(
    State(state): State<Arc<RwLock<AppState>>>,
    Query(params): Query<ListDefinitionsParams>,
) -> Result<Json<Vec<NodeDefinitionResponse>>> {
    let state = state.read().await;
    
    let definitions: Vec<NodeDefinition> = if let Some(kind) = &params.kind {
        sqlx::query_as::<_, NodeDefinition>(
            r#"SELECT id, node_id, name, description, kind, device_type, 
               input_schema, output_schema, config_schema, default_config, icon, color, 
               category, is_system, created_at, updated_at 
               FROM ses_node_definitions 
               WHERE kind = $1
               ORDER BY category, name"#
        )
        .bind(kind)
        .fetch_all(&state.db_pool)
        .await?
    } else {
        sqlx::query_as::<_, NodeDefinition>(
            r#"SELECT id, node_id, name, description, kind, device_type, 
               input_schema, output_schema, config_schema, default_config, icon, color, 
               category, is_system, created_at, updated_at 
               FROM ses_node_definitions 
               ORDER BY category, name"#
        )
        .fetch_all(&state.db_pool)
        .await?
    };
    
    let responses: Vec<NodeDefinitionResponse> = definitions.into_iter()
        .map(|d| d.into())
        .collect();
    
    Ok(Json(responses))
}

async fn create_definition(
    State(_state): State<Arc<RwLock<AppState>>>,
    Json(_request): Json<CreateNodeDefinitionRequest>,
) -> Result<Json<NodeDefinitionResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn get_definition(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<NodeDefinitionResponse>> {
    let state = state.read().await;
    
    let definition = sqlx::query_as::<_, NodeDefinition>(
        r#"SELECT id, node_id, name, description, kind, device_type, 
           input_schema, output_schema, config_schema, default_config, icon, color, 
           category, is_system, created_at, updated_at 
           FROM ses_node_definitions WHERE id = $1"#
    )
    .bind(id)
    .fetch_optional(&state.db_pool)
    .await?;
    
    match definition {
        Some(def) => Ok(Json(def.into())),
        None => Err(crate::core::error::SesError::NotFound(format!("Node definition not found: {}", id))),
    }
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
