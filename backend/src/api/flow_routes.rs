use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::core::state::AppState;
use crate::core::types::PageRequest;
use crate::core::error::Result;
use crate::models::flow::{
    CreateFlowRequest, ExecuteFlowRequest, FlowResponse, FlowInstanceResponse,
    UpdateFlowRequest, FlowRow, FlowInstanceRow, FlowInstance,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_flows).post(create_flow))
        .route("/:id", get(get_flow).put(update_flow).delete(delete_flow))
        .route("/:id/execute", post(execute_flow))
        .route("/:id/instances", get(list_instances))
        .route("/:instance_id/resume", post(resume_flow))
        .route("/:instance_id/cancel", post(cancel_flow))
}

#[derive(Debug, Deserialize)]
struct ListFlowsParams {
    app_id: Option<Uuid>,
    page: Option<i64>,
    size: Option<i64>,
}

async fn list_flows(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListFlowsParams>,
) -> Result<Json<Vec<FlowResponse>>> {
    let flows = sqlx::query_as::<_, FlowRow>(
        r#"
        SELECT id, app_id, name, description, flow_json, version, is_template, 
               status, created_at, updated_at, created_by
        FROM ses_flows 
        WHERE ($1::uuid IS NULL OR app_id = $1)
        ORDER BY updated_at DESC
        "#
    )
    .bind(params.app_id)
    .fetch_all(&state.db_pool)
    .await?;
    
    let responses: Vec<FlowResponse> = flows.into_iter()
        .map(|row| row.into())
        .collect();
    
    Ok(Json(responses))
}

async fn create_flow(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateFlowRequest>,
) -> Result<Json<FlowResponse>> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let app_id = Uuid::nil();
    
    let flow_json = serde_json::to_value(&request.flow_json)?;
    
    let row = sqlx::query_as::<_, FlowRow>(
        r#"
        INSERT INTO ses_flows (id, app_id, name, description, flow_json, version, is_template, status, created_at, updated_at, created_by)
        VALUES ($1, $2, $3, $4, $5, 1, $6, 'DRAFT', $7, $7, $8)
        RETURNING id, app_id, name, description, flow_json, version, is_template, 
                  status, created_at, updated_at, created_by
        "#
    )
    .bind(id)
    .bind(app_id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&flow_json)
    .bind(request.is_template.unwrap_or(false))
    .bind(now)
    .bind(None::<String>)
    .fetch_one(&state.db_pool)
    .await?;
    
    Ok(Json(row.into()))
}

async fn get_flow(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<FlowResponse>> {
    let row = sqlx::query_as::<_, FlowRow>(
        r#"
        SELECT id, app_id, name, description, flow_json, version, is_template, 
               status, created_at, updated_at, created_by
        FROM ses_flows 
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(&state.db_pool)
    .await?;
    
    match row {
        Some(flow) => Ok(Json(flow.into())),
        None => Err(crate::core::error::SesError::NotFound(format!("Flow not found: {}", id))),
    }
}

async fn update_flow(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateFlowRequest>,
) -> Result<Json<FlowResponse>> {
    let now = Utc::now();
    
    let existing = sqlx::query_as::<_, FlowRow>(
        r#"
        SELECT id, app_id, name, description, flow_json, version, is_template, 
               status, created_at, updated_at, created_by
        FROM ses_flows 
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(&state.db_pool)
    .await?;
    
    if existing.is_none() {
        return Err(crate::core::error::SesError::NotFound(format!("Flow not found: {}", id)));
    }
    
    let name = request.name.as_ref().map(|n| n.as_str());
    let description = request.description.as_ref().map(|d| d.as_str());
    let flow_json = request.flow_json.map(|f| serde_json::to_value(f).ok()).flatten();
    
    let row = sqlx::query_as::<_, FlowRow>(
        r#"
        UPDATE ses_flows 
        SET name = COALESCE($2, name),
            description = COALESCE($3, description),
            flow_json = COALESCE($4, flow_json),
            status = COALESCE($5, status),
            updated_at = $6,
            version = version + 1
        WHERE id = $1
        RETURNING id, app_id, name, description, flow_json, version, is_template, 
                  status, created_at, updated_at, created_by
        "#
    )
    .bind(id)
    .bind(name)
    .bind(description)
    .bind(flow_json)
    .bind(request.status)
    .bind(now)
    .fetch_one(&state.db_pool)
    .await?;
    
    Ok(Json(row.into()))
}

async fn delete_flow(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    let result = sqlx::query("DELETE FROM ses_flows WHERE id = $1")
        .bind(id)
        .execute(&state.db_pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(crate::core::error::SesError::NotFound(format!("Flow not found: {}", id)));
    }
    
    Ok(Json(serde_json::json!({ "success": true, "id": id })))
}

async fn execute_flow(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(_request): Json<ExecuteFlowRequest>,
) -> Result<Json<FlowInstanceResponse>> {
    let now = Utc::now();
    let instance_id = Uuid::new_v4();
    
    let row = sqlx::query_as::<_, FlowInstanceRow>(
        r#"
        INSERT INTO ses_flow_instances (id, flow_id, app_id, status, context, started_at, error_message, created_at, updated_at)
        VALUES ($1, $2, $3, 'RUNNING', $4, $5, NULL, $6, $6)
        RETURNING id, flow_id, app_id, status, context, started_at, completed_at, error_message, created_at, updated_at
        "#
    )
    .bind(instance_id)
    .bind(id)
    .bind(Uuid::nil())
    .bind(None::<serde_json::Value>)
    .bind(now)
    .bind(now)
    .fetch_one(&state.db_pool)
    .await?;
    
    let instance: FlowInstance = row.into();
    Ok(Json(instance.into()))
}

async fn list_instances(
    State(state): State<Arc<AppState>>,
    Path(flow_id): Path<Uuid>,
    Query(_params): Query<PageRequest>,
) -> Result<Json<Vec<FlowInstanceResponse>>> {
    let rows = sqlx::query_as::<_, FlowInstanceRow>(
        r#"
        SELECT id, flow_id, app_id, status, context, 
               started_at, completed_at, error_message, created_at, updated_at
        FROM ses_flow_instances 
        WHERE flow_id = $1
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .bind(flow_id)
    .fetch_all(&state.db_pool)
    .await?;
    
    let responses: Vec<FlowInstanceResponse> = rows.into_iter()
        .map(|r| {
            let instance: FlowInstance = r.into();
            instance.into()
        })
        .collect();
    
    Ok(Json(responses))
}

async fn resume_flow(
    State(_state): State<Arc<AppState>>,
    Path(_instance_id): Path<Uuid>,
) -> Result<Json<FlowInstanceResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn cancel_flow(
    State(_state): State<Arc<AppState>>,
    Path(_instance_id): Path<Uuid>,
) -> Result<Json<FlowInstanceResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}
