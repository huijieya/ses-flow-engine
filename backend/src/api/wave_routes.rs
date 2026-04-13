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
use crate::models::stats::SesResponse;
use crate::models::wave::{
    CreateWaveRequest, Wave, WaveResponse, WaveStatus,
};
use crate::models::order::{
    CreateOrderRequest, OrderResponse, OrderType, OrderStatus,
};

/// Wave management routes - SES 1.0 compatible
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        // Wave operations
        .route("/add", post(add_wave))
        .route("/close/:waveId", get(close_wave))
        .route("/close/:waveId/:orderId", get(close_order))
        .route("/start/:waveId", get(start_wave))
        .route("/cancel/:waveId", get(cancel_wave))
        // Query operations
        .route("/list", get(list_waves))
        .route("/detail/:waveId", get(get_wave_detail))
        // Order operations
        .route("/:waveId/orders/add", post(add_order_to_wave))
        .route("/:waveId/orders/:orderId/cancel", post(cancel_order))
}

/// Add new wave - triggers wave_create flow
async fn add_wave(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateWaveRequest>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;
    let now = Utc::now();
    let id = Uuid::new_v4();

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

    tracing::info!("Wave {} created, triggering flow engine", req.wave_id);
    Ok(Json(SesResponse::success(wave)))
}

/// Close wave - triggers wave_close flow
async fn close_wave(
    State(state): State<Arc<AppState>>,
    Path(wave_id): Path<String>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;

    let result = sqlx::query(
        "UPDATE ses_waves SET status = 'CLOSED', completed_at = NOW(), updated_at = NOW() WHERE wave_id = $1",
    )
    .bind(&wave_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(SesError::NotFound(format!("Wave not found: {}", wave_id)));
    }

    tracing::info!("Wave {} closed, triggering flow engine", wave_id);
    Ok(Json(SesResponse::success(())))
}

/// Close order in wave - triggers order_close flow
async fn close_order(
    State(state): State<Arc<AppState>>,
    Path((wave_id, order_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;

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

    tracing::info!("Order {} in wave {} closed, triggering flow engine", order_id, wave_id);
    Ok(Json(SesResponse::success(())))
}

/// Start wave - triggers wave_start flow
async fn start_wave(
    State(state): State<Arc<AppState>>,
    Path(wave_id): Path<String>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;

    let result = sqlx::query(
        "UPDATE ses_waves SET status = 'STARTED', started_at = NOW(), updated_at = NOW() WHERE wave_id = $1",
    )
    .bind(&wave_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(SesError::NotFound(format!("Wave not found: {}", wave_id)));
    }

    tracing::info!("Wave {} started, triggering flow engine", wave_id);
    Ok(Json(SesResponse::success(())))
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

    Ok(Json(SesResponse::success(())))
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

/// Add order to wave
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

    // Insert order details
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

    // Update wave order count
    sqlx::query(
        "UPDATE ses_waves SET total_orders = total_orders + 1, updated_at = NOW() WHERE wave_id = $1",
    )
    .bind(&wave_id)
    .execute(pool)
    .await?;

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

    Ok(Json(SesResponse::success(())))
}
