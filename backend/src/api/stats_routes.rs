use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Local;
use std::sync::Arc;

use crate::core::state::AppState;
use crate::models::stats::{
    BaseQuery, HomePageVo, PageVo, SortEfficiencyVo, StationSortVo,
    SesResponse, SystemStatusVo, WaveDayVo, WaveExecVo, WaveTypeVo,
};

/// Stats routes
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/waveInfo/home/page", get(home_page))
        .route("/waveInfo/home/page/wave", post(home_page_waves))
        .route("/waveInfo/home/page/station", post(home_page_stations))
        .route("/waveInfo/type", get(get_wave_types))
}

/// Get home page base info
async fn home_page(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    let wave_day = WaveDayVo {
        twave_completed_compare_yest: 0,
        twave_completed: 5,
        twave_qty: 8,
        twave_qty_compare_yest: 0,
    };

    let system_status = SystemStatusVo {
        wave_day_po: wave_day,
        twave_completed_compare_yest: 0,
        twave_completed: 15,
        tsorting_qty_compare_yest: 0,
        tsorting_qty: 1250,
        tpack_qty_compare_yest: 0,
        twave_qty: 20,
        twave_qty_compare_yest: 0,
        tpack_qty: 980,
    };

    let now = Local::now();
    let times: Vec<String> = (0..12)
        .map(|i| now - chrono::Duration::hours(i))
        .map(|t| t.format("%H:00").to_string())
        .rev()
        .collect();

    let efficiencies = vec![85, 90, 88, 92, 87, 91, 89, 93, 90, 88, 91, 89];
    let avg = efficiencies.iter().sum::<i32>() / efficiencies.len() as i32;
    let high = *efficiencies.iter().max().unwrap_or(&0);
    let low = *efficiencies.iter().min().unwrap_or(&0);

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

    Json(SesResponse::success(home_page))
}

/// Get home page wave list
async fn home_page_waves(
    State(_state): State<Arc<AppState>>,
    Json(query): Json<BaseQuery>,
) -> impl IntoResponse {
    let _page_size = query.page_size.max(1).min(100);
    let _offset = (query.page_num.max(0) * _page_size) as i64;

    let waves = vec![
        WaveExecVo {
            wave_id: "WV001".to_string(),
            create_time: Some(chrono::Utc::now()),
            completed_time: None,
            order_qty: 50,
            all_qty: 200,
            speed: 15.5,
            wave_status: "STARTED".to_string(),
            wave_status_name: "进行中".to_string(),
            rate: 60,
        },
    ];

    let page = PageVo {
        total: 1,
        data: waves,
    };

    Json(SesResponse::success(page))
}

/// Get home page station list
async fn home_page_stations(
    State(_state): State<Arc<AppState>>,
    Json(query): Json<BaseQuery>,
) -> impl IntoResponse {
    let _page_size = query.page_size.max(1).min(100);
    let _offset = (query.page_num.max(0) * _page_size) as i64;

    let stations = vec![
        StationSortVo {
            station_id: "ST001".to_string(),
            sort_qty: 120,
            efficiency: 95.5,
            station_status_name: "启用".to_string(),
            sort_efficiency: 92,
            station_status: "enable".to_string(),
        },
    ];

    let page = PageVo {
        total: 1,
        data: stations,
    };

    Json(SesResponse::success(page))
}

/// Get wave types
async fn get_wave_types() -> impl IntoResponse {
    let types = vec![
        WaveTypeVo { wave_type: 0, label: "正向分拣".to_string() },
        WaveTypeVo { wave_type: 1, label: "逆向退货".to_string() },
        WaveTypeVo { wave_type: 2, label: "调拨分拣".to_string() },
    ];

    Json(SesResponse::success(types))
}
