use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Local;
use std::sync::Arc;

use crate::core::error::SesError;
use crate::core::state::AppState;
use crate::models::stats::{
    BaseQuery, HomePageVo, PageVo, SortEfficiencyVo, StationSortVo,
    SesResponse, SystemStatusVo, WaveDayVo, WaveExecVo, WaveTypeVo,
};

/// Stats routes - data query class, direct REST API with real database queries
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/waveInfo/home/page", get(home_page))
        .route("/waveInfo/home/page/wave", post(home_page_waves))
        .route("/waveInfo/home/page/station", post(home_page_stations))
        .route("/waveInfo/type", get(get_wave_types))
}

/// Get home page base info - queries real database for statistics
async fn home_page(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;

    // Query wave statistics from database
    let wave_stats: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            COALESCE(SUM(total_orders), 0),
            COALESCE(SUM(completed_orders), 0),
            COALESCE(COUNT(*) FILTER (WHERE status = 'STARTED'), 0),
            COALESCE(COUNT(*) FILTER (WHERE status = 'CLOSED' AND completed_at::date = CURRENT_DATE), 0)
        FROM ses_waves
        "#
    )
    .fetch_one(pool)
    .await
    .unwrap_or((0, 0, 0, 0));

    let total_orders = wave_stats.0;
    let completed_orders = wave_stats.1;
    let active_waves = wave_stats.2;
    let today_completed = wave_stats.3;

    // Query today's sorting/packing statistics
    let sort_stats: (i64, i64) = sqlx::query_as(
        r#"
        SELECT
            COALESCE(COUNT(*) FILTER (WHERE status = 'CLOSED'), 0),
            COALESCE(COUNT(*), 0)
        FROM ses_orders
        WHERE created_at::date = CURRENT_DATE
        "#
    )
    .fetch_one(pool)
    .await
    .unwrap_or((0, 0));

    let wave_day = WaveDayVo {
        twave_completed_compare_yest: 0,
        twave_completed: today_completed as i32,
        twave_qty: active_waves as i32,
        twave_qty_compare_yest: 0,
    };

    let system_status = SystemStatusVo {
        wave_day_po: wave_day,
        twave_completed_compare_yest: 0,
        twave_completed: today_completed as i32,
        tsorting_qty_compare_yest: 0,
        tsorting_qty: sort_stats.0 as i64,
        tpack_qty_compare_yest: 0,
        twave_qty: (active_waves + today_completed) as i32,
        twave_qty_compare_yest: 0,
        tpack_qty: completed_orders as i64,
    };

    // Generate hourly efficiency data
    let now = Local::now();
    let times: Vec<String> = (0..12)
        .map(|i| now - chrono::Duration::hours(i))
        .map(|t| t.format("%H:00").to_string())
        .rev()
        .collect();

    // Query hourly sorting efficiency from database
    let efficiencies: Vec<i32> = sqlx::query_scalar::<_, i32>(
        r#"
        WITH hours AS (
            SELECT generate_series(
                date_trunc('hour', NOW() - INTERVAL '11 hours'),
                date_trunc('hour', NOW()),
                INTERVAL '1 hour'
            ) AS hour
        )
        SELECT COALESCE(
            (SELECT COUNT(*) FROM ses_orders
             WHERE created_at >= h.hour
               AND created_at < h.hour + INTERVAL '1 hour'
               AND status = 'CLOSED'
            ) * 100 / NULLIF(
                (SELECT COUNT(*) FROM ses_orders
                 WHERE created_at >= h.hour
                   AND created_at < h.hour + INTERVAL '1 hour'
                ), 0
            ),
            0
        )
        FROM hours h
        ORDER BY h.hour
        "#
    )
    .fetch_all(pool)
    .await
    .unwrap_or_else(|_| vec![0; 12]);

    let avg = if efficiencies.is_empty() { 0 } else { efficiencies.iter().sum::<i32>() / efficiencies.len() as i32 };
    let high = efficiencies.iter().max().copied().unwrap_or(0);
    let low = efficiencies.iter().min().copied().unwrap_or(0);

    let sort_efficiency = SortEfficiencyVo {
        times,
        efficiencies,
        avg_efficiency: avg,
        high_efficiency: high,
        low_efficiency: low,
    };

    let home_page = HomePageVo {
        system_status_vo: system_status,
        sort_efficiency_vo: sort_efficiency,
    };

    Ok(Json(SesResponse::success(home_page)))
}

/// Get home page wave list - queries real database
async fn home_page_waves(
    State(state): State<Arc<AppState>>,
    Json(query): Json<BaseQuery>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;
    let page_size = query.page_size.max(1).min(100);
    let offset = (query.page_num.max(0) * page_size) as i64;

    let waves = sqlx::query_as::<_, WaveExecVo>(
        r#"
        SELECT
            wave_id,
            created_at,
            completed_at,
            total_orders AS order_qty,
            completed_orders AS all_qty,
            COALESCE(
                CASE WHEN total_orders > 0
                    THEN (completed_orders::float / total_orders::float) * 100
                    ELSE 0
                END, 0
            )::int AS rate,
            status AS wave_status,
            CASE status
                WHEN 'CREATED' THEN '已创建'
                WHEN 'STARTED' THEN '进行中'
                WHEN 'CLOSED' THEN '已完成'
                WHEN 'CANCELLED' THEN '已取消'
                ELSE status
            END AS wave_status_name,
            COALESCE(
                CASE WHEN EXTRACT(EPOCH FROM (NOW() - started_at)) > 0
                    THEN completed_orders::float / EXTRACT(EPOCH FROM (NOW() - started_at)) * 3600
                    ELSE 0
                END, 0
            ) AS speed
        FROM ses_waves
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(page_size as i64)
    .bind(offset)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ses_waves")
        .fetch_one(pool)
        .await
        .unwrap_or(0);

    let page = PageVo {
        total,
        data: waves,
    };

    Ok(Json(SesResponse::success(page)))
}

/// Get home page station list - queries real database
async fn home_page_stations(
    State(state): State<Arc<AppState>>,
    Json(query): Json<BaseQuery>,
) -> Result<impl IntoResponse, SesError> {
    let pool = &state.db_pool;
    let page_size = query.page_size.max(1).min(100);
    let offset = (query.page_num.max(0) * page_size) as i64;

    let stations = sqlx::query_as::<_, StationSortVo>(
        r#"
        SELECT
            s.station_id,
            COALESCE(o.sort_qty, 0) AS sort_qty,
            COALESCE(o.sort_qty::float / NULLIF(EXTRACT(EPOCH FROM (NOW() - s.created_at)) / 3600, 0), 0) AS efficiency,
            CASE s.status
                WHEN 'enable' THEN '启用'
                WHEN 'disable' THEN '停用'
                ELSE s.status
            END AS station_status_name,
            COALESCE((o.sort_qty::float / NULLIF(o.total_qty, 0)) * 100, 0)::int AS sort_efficiency,
            s.status AS station_status
        FROM ses_stations s
        LEFT JOIN (
            SELECT
                wave_id,
                COUNT(*) AS sort_qty,
                COUNT(*) AS total_qty
            FROM ses_orders
            WHERE status = 'CLOSED'
            GROUP BY wave_id
        ) o ON s.wave_id = o.wave_id
        ORDER BY s.created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(page_size as i64)
    .bind(offset)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ses_stations")
        .fetch_one(pool)
        .await
        .unwrap_or(0);

    let page = PageVo {
        total,
        data: stations,
    };

    Ok(Json(SesResponse::success(page)))
}

/// Get wave types - static enum data
async fn get_wave_types() -> Result<impl IntoResponse, SesError> {
    let types = vec![
        WaveTypeVo { wave_type: 0, label: "正向分拣".to_string() },
        WaveTypeVo { wave_type: 1, label: "逆向退货".to_string() },
        WaveTypeVo { wave_type: 2, label: "调拨分拣".to_string() },
    ];

    Ok(Json(SesResponse::success(types)))
}
