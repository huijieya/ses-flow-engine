use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::core::state::AppState;
use crate::core::error::SesError;
use crate::core::types::ExecutionContext;
use crate::models::stats::SesResponse;
use crate::models::wave::{
    CreateWaveRequest, Wave, WaveResponse, WaveStatus,
};
use crate::models::order::{
    CreateOrderRequest, OrderResponse, OrderType, OrderStatus,
};

/// Wave management routes - SES 1.0 compatible, internally uses flow orchestration
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/add", post(add_wave))
        .route("/close/:waveId", get(close_wave))
        .route("/close/:waveId/:orderId", get(close_order))
        .route("/start/:waveId", get(start_wave))
        .route("/cancel/:waveId", get(cancel_wave))
        .route("/list", get(list_waves))
        .route("/detail/:waveId", get(get_wave_detail))
        .route("/:waveId/orders/add", post(add_order_to_wave))
        .route("/:waveId/orders/:orderId/cancel", post(cancel_order))
}

/// Add new wave - triggers wave_create orchestration node
async fn add_wave(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateWaveRequest>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;
    let now = Utc::now();
    let id = Uuid::new_v4();

    // 1. Persist wave to database
    let wave = sqlx::query_as::<_, Wave>(
        r#"
        INSERT INTO ses_waves (id, wave_id, app_id, wave_name, priority, status, platform_id, total_orders, completed_orders, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, 0, 0, $8, $8)
        RETURNING id, wave_id, app_id, wave_name, priority, status, platform_id, total_orders, completed_orders, started_at, completed_at, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(&req.wave_id)
    .bind(Uuid::nil())
    .bind(&req.wave_name)
    .bind(req.priority.unwrap_or(0))
    .bind(WaveStatus::Created.as_str())
    .bind(&req.platform_id)
    .bind(now)
    .fetch_one(pool)
    .await?;

    // 2. Trigger wave_create node via flow engine
    let mut ctx = ExecutionContext::new();
    ctx.set("wave_id", &req.wave_id);
    ctx.set("wave_type", req.wave_type.as_deref().unwrap_or("normal"));
    ctx.set("priority", req.priority.unwrap_or(0));

    match state.flow_engine.execute_node_direct("wave_create", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx).await {
        Ok(result) => {
            tracing::info!("Wave {} created via flow engine: {:?}", req.wave_id, result.output);
        }
        Err(e) => {
            tracing::warn!("Flow engine wave_create failed (wave still persisted): {}", e);
        }
    }

    // 3. Publish event
    let _ = state.event_bus.publish_simple("wave.created", &serde_json::json!({
        "wave_id": req.wave_id,
        "wave_name": req.wave_name,
    })).await;

    Ok(Json(SesResponse::success(wave)))
}

/// Start wave - triggers wave_start orchestration node
async fn start_wave(
    State(state): State<Arc<AppState>>,
    Path(wave_id): Path<String>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;

    // 1. Update wave status in database
    let result = sqlx::query(
        "UPDATE ses_waves SET status = 'STARTED', started_at = NOW(), updated_at = NOW() WHERE wave_id = $1",
    )
    .bind(&wave_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(SesError::NotFound(format!("Wave not found: {}", wave_id)));
    }

    // 2. Trigger wave_start node via flow engine
    let mut ctx = ExecutionContext::new();
    ctx.set("wave_id", &wave_id);

    // Query stations assigned to this wave
    let stations: Vec<String> = sqlx::query_scalar::<_, String>(
        "SELECT station_id FROM ses_wave_stations WHERE wave_id = $1",
    )
    .bind(&wave_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    ctx.set("station_ids", &stations);

    match state.flow_engine.execute_node_direct("wave_start", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx).await {
        Ok(result) => {
            tracing::info!("Wave {} started via flow engine: {:?}", wave_id, result.output);
        }
        Err(e) => {
            tracing::warn!("Flow engine wave_start failed (wave status still updated): {}", e);
        }
    }

    // 3. Publish event
    let _ = state.event_bus.publish_simple("wave.started", &serde_json::json!({
        "wave_id": wave_id,
        "station_count": stations.len(),
    })).await;

    Ok(Json(SesResponse::success(serde_json::json!({
        "wave_id": wave_id,
        "status": "STARTED",
        "stations_ready": stations.len()
    }))))
}

/// Close wave - triggers wave_close orchestration node
async fn close_wave(
    State(state): State<Arc<AppState>>,
    Path(wave_id): Path<String>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;

    // 1. Update wave status
    let result = sqlx::query(
        "UPDATE ses_waves SET status = 'CLOSED', completed_at = NOW(), updated_at = NOW() WHERE wave_id = $1",
    )
    .bind(&wave_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(SesError::NotFound(format!("Wave not found: {}", wave_id)));
    }

    // 2. Trigger wave_close node
    let mut ctx = ExecutionContext::new();
    ctx.set("wave_id", &wave_id);
    ctx.set("close_type", "normal");

    // Get order stats
    let (total, completed): (i64, i64) = sqlx::query_as(
        "SELECT COALESCE(COUNT(*), 0), COALESCE(SUM(CASE WHEN status = 'CLOSED' THEN 1 ELSE 0 END), 0) FROM ses_orders WHERE wave_id = $1",
    )
    .bind(&wave_id)
    .fetch_one(pool)
    .await
    .unwrap_or((0, 0));

    ctx.set("total_orders", total);
    ctx.set("completed_orders", completed);

    match state.flow_engine.execute_node_direct("wave_close", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx).await {
        Ok(result) => {
            tracing::info!("Wave {} closed via flow engine: {:?}", wave_id, result.output);
        }
        Err(e) => {
            tracing::warn!("Flow engine wave_close failed: {}", e);
        }
    }

    // 3. Publish event
    let _ = state.event_bus.publish_simple("wave.closed", &serde_json::json!({
        "wave_id": wave_id,
        "total_orders": total,
        "completed_orders": completed,
    })).await;

    Ok(Json(SesResponse::success(serde_json::json!({
        "wave_id": wave_id,
        "status": "CLOSED",
        "total_orders": total,
        "completed_orders": completed,
    }))))
}

/// Close order in wave - triggers order_close orchestration node
async fn close_order(
    State(state): State<Arc<AppState>>,
    Path((wave_id, order_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;

    // 1. Update order status
    let result = sqlx::query(
        "UPDATE ses_orders SET status = 'CLOSED', updated_at = NOW() WHERE order_id = $1 AND wave_id = $2",
    )
    .bind(&order_id)
    .bind(&wave_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(SesError::NotFound(format!("Order not found: {} in wave {}", order_id, wave_id)));
    }

    // 2. Update wave completed count
    sqlx::query(
        "UPDATE ses_waves SET completed_orders = completed_orders + 1, updated_at = NOW() WHERE wave_id = $1",
    )
    .bind(&wave_id)
    .execute(pool)
    .await?;

    // 3. Trigger order_close node
    let mut ctx = ExecutionContext::new();
    ctx.set("order_id", &order_id);
    ctx.set("wave_id", &wave_id);

    match state.flow_engine.execute_node_direct("order_close", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx).await {
        Ok(result) => {
            tracing::info!("Order {} closed via flow engine: {:?}", order_id, result.output);
        }
        Err(e) => {
            tracing::warn!("Flow engine order_close failed: {}", e);
        }
    }

    // 4. Publish event
    let _ = state.event_bus.publish_simple("order.closed", &serde_json::json!({
        "order_id": order_id,
        "wave_id": wave_id,
    })).await;

    Ok(Json(SesResponse::success(serde_json::json!({
        "order_id": order_id,
        "wave_id": wave_id,
        "status": "CLOSED",
    }))))
}

/// Cancel wave
async fn cancel_wave(
    State(state): State<Arc<AppState>>,
    Path(wave_id): Path<String>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;

    let result = sqlx::query(
        "UPDATE ses_waves SET status = 'CANCELLED', updated_at = NOW() WHERE wave_id = $1",
    )
    .bind(&wave_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(SesError::NotFound(format!("Wave not found: {}", wave_id)));
    }

    // Cancel all active orders in this wave
    sqlx::query(
        "UPDATE ses_orders SET status = 'CANCELLED', updated_at = NOW() WHERE wave_id = $1 AND status NOT IN ('CLOSED', 'CANCELLED')",
    )
    .bind(&wave_id)
    .execute(pool)
    .await?;

    let _ = state.event_bus.publish_simple("wave.cancelled", &serde_json::json!({
        "wave_id": wave_id,
    })).await;

    Ok(Json(SesResponse::success(serde_json::json!({
        "wave_id": wave_id,
        "status": "CANCELLED",
    }))))
}

#[derive(Debug, Deserialize)]
struct ListWavesQuery {
    page: Option<i64>,
    size: Option<i64>,
    status: Option<String>,
}

/// List waves
async fn list_waves(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListWavesQuery>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;
    let limit = query.size.unwrap_or(20).max(1).min(100);
    let offset = query.page.unwrap_or(0).max(0) * limit;

    let waves = sqlx::query_as::<_, Wave>(
        r#"
        SELECT id, wave_id, app_id, wave_name, priority, status, platform_id, total_orders, completed_orders, started_at, completed_at, created_at, updated_at
        FROM ses_waves
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let responses: Vec<WaveResponse> = waves.into_iter().map(|w| w.into()).collect();
    Ok(Json(SesResponse::success(responses)))
}

/// Get wave detail
async fn get_wave_detail(
    State(state): State<Arc<AppState>>,
    Path(wave_id): Path<String>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;

    let wave = sqlx::query_as::<_, Wave>(
        r#"
        SELECT id, wave_id, app_id, wave_name, priority, status, platform_id, total_orders, completed_orders, started_at, completed_at, created_at, updated_at
        FROM ses_waves
        WHERE wave_id = $1
        "#,
    )
    .bind(&wave_id)
    .fetch_optional(pool)
    .await?;

    match wave {
        Some(w) => Ok(Json(SesResponse::success(w))),
        None => Err(SesError::NotFound(format!("Wave not found: {}", wave_id))),
    }
}

/// Add order to wave - triggers order_assign node
async fn add_order_to_wave(
    State(state): State<Arc<AppState>>,
    Path(wave_id): Path<String>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;
    let now = Utc::now();
    let id = Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO ses_orders (id, order_id, app_id, wave_id, order_type, chute_id, status, priority, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9)
        "#,
    )
    .bind(id)
    .bind(&req.order_id)
    .bind(Uuid::nil())
    .bind(&wave_id)
    .bind(req.order_type.as_str())
    .bind(&req.chute_id)
    .bind(OrderStatus::UnStarted.as_str())
    .bind(req.priority.unwrap_or(0))
    .bind(now)
    .execute(pool)
    .await?;

    for detail in &req.details {
        let detail_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO ses_order_details (id, order_id, sku, barcode, qty, completed_qty, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 0, $6, $6)",
        )
        .bind(detail_id)
        .bind(&req.order_id)
        .bind(&detail.sku)
        .bind(&detail.barcode)
        .bind(detail.qty)
        .bind(now)
        .execute(pool)
        .await?;
    }

    sqlx::query(
        "UPDATE ses_waves SET total_orders = total_orders + 1, updated_at = NOW() WHERE wave_id = $1",
    )
    .bind(&wave_id)
    .execute(pool)
    .await?;

    // Trigger order_assign node
    let mut ctx = ExecutionContext::new();
    ctx.set("order_id", &req.order_id);

    match state.flow_engine.execute_node_direct("order_assign", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx).await {
        Ok(result) => {
            tracing::info!("Order {} assigned via flow engine: {:?}", req.order_id, result.output);
        }
        Err(e) => {
            tracing::warn!("Flow engine order_assign failed: {}", e);
        }
    }

    let response = OrderResponse {
        id,
        order_id: req.order_id,
        wave_id,
        order_type: req.order_type,
        chute_id: req.chute_id,
        status: OrderStatus::UnStarted,
        priority: req.priority.unwrap_or(0),
        created_at: now,
    };

    Ok(Json(SesResponse::success(response)))
}

/// Cancel order
async fn cancel_order(
    State(state): State<Arc<AppState>>,
    Path((wave_id, order_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;

    let result = sqlx::query(
        "UPDATE ses_orders SET status = 'CANCELLED', updated_at = NOW() WHERE order_id = $1 AND wave_id = $2",
    )
    .bind(&order_id)
    .bind(&wave_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(SesError::NotFound(format!("Order not found: {} in wave {}", order_id, wave_id)));
    }

    Ok(Json(SesResponse::success(serde_json::json!({
        "order_id": order_id,
        "status": "CANCELLED",
    }))))
}
