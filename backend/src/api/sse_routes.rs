//! SSE routes for station and RCS connections
//! SSE连接路由

use axum::{
    body::{Body, Bytes},
    extract::{Path, Query, State},
    http::{header, HeaderValue},
    response::Response,
    response::sse::{Event, Sse},
    routing::post,
    routing::get,
    Json,
    Router,
};
use futures::stream::{self, Stream};
use futures::StreamExt;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;


use crate::core::state::AppState;
use crate::core::error::Result;
use crate::sse::{SseEventType, SseBroadcastEvent, SseTarget};
use crate::models::station::ConnectRequestDto;

/// Query params for station SSE connection
#[derive(Debug, Deserialize)]
pub struct StationSseQuery {
    pub platform_id: String,
}

/// Query params for RCS SSE connection
#[derive(Debug, Deserialize)]
pub struct RcsSseQuery {
    pub rcs_id: String,
    pub platform_ids: String, // comma-separated
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        // Station SSE connection
        .route("/station/operation/connect/:station_id", get(station_sse_handler))
        .route("/station/operation/connect", post(station_sse_post_handler))
        // RCS SSE connection
        .route("/rcs/operation/sse/subscribe", get(rcs_sse_handler))
        // 联调触发器：手动发送 AGV 到达事件
        .route("/station/operation/test/triggerAgvArrived", post(trigger_agv_arrived))
}

/// Station SSE connection handler
/// GET /api/v1/sse/station/operation/connect/:station_id?platform_id=xxx
async fn station_sse_handler(
    State(state): State<Arc<AppState>>,
    Path(station_id): Path<String>,
    Query(query): Query<StationSseQuery>,
) -> Sse<impl Stream<Item = std::result::Result<Event, BroadcastStreamRecvError>>> {
    tracing::info!(
        "Station {} establishing SSE connection on platform {}",
        station_id,
        query.platform_id
    );

    // Register station connection
    let receiver = state
        .sse_manager
        .register_station(station_id.clone(), query.platform_id.clone())
        .await;

    // Create stream from receiver
    // Create stream from receiver
    let station_id_for_stream = station_id.clone(); // 1. 克隆一份给流使用
    let stream = BroadcastStream::new(receiver)
        .filter_map(move |result| { 
            let station_id = station_id_for_stream.clone(); 
            async move {
                match result {
                    Ok(event) => Some(Ok(event.to_sse_event())),
                    Err(e) => {
                        tracing::warn!("SSE receive error for station {}: {}", station_id, e);
                        Some(Err(e))
                    }
                }
            }
        });

    // Return SSE response with keep-alive
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive"),
    )
}

/// POST station SSE connection handler for workstation compatibility
/// POST /api/v1/sse/station/operation/connect
async fn station_sse_post_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ConnectRequestDto>,
) -> Response<Body> {
    let station_id = body
        .station_ids
        .first()
        .cloned()
        .unwrap_or_else(|| body.client_id.clone());

    tracing::info!(
        "Station {} establishing POST SSE connection on platform {}",
        station_id,
        body.platform_id
    );

    let receiver = state
        .sse_manager
        .register_station(station_id.clone(), body.platform_id)
        .await;

    let stream = BroadcastStream::new(receiver).filter_map(move |result| { 
        let value = station_id.clone();
        async move {
            let value = value.clone();
        match result {
            Ok(event) => {
                let frame = format!("data: {}\n\n", event.payload_string());
                Some(Ok::<Bytes, Infallible>(Bytes::from(frame)))
            }
            Err(e) => {
                tracing::warn!("POST SSE receive error for station {}: {}", value, e);
                None
            }
        }
    }});

    let mut response = Response::new(Body::from_stream(stream));
    response
        .headers_mut()
        .insert(header::CONTENT_TYPE, HeaderValue::from_static("text/event-stream"));
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    response
        .headers_mut()
        .insert(header::CONNECTION, HeaderValue::from_static("keep-alive"));
    response
}

/// RCS SSE connection handler
/// GET /api/v1/sse/rcs/operation/sse/subscribe?rcs_id=xxx&platform_ids=pid1,pid2
async fn rcs_sse_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RcsSseQuery>,
) -> Sse<impl Stream<Item = std::result::Result<Event, BroadcastStreamRecvError>>> {
    let platform_ids: Vec<String> = query
        .platform_ids
        .split(',')
        .map(|s| s.to_string())
        .collect();

    tracing::info!(
        "RCS {} establishing SSE connection for platforms {:?}",
        query.rcs_id,
        platform_ids
    );

    // Register RCS connection
    let receiver = state
        .sse_manager
        .register_rcs(query.rcs_id.clone(), platform_ids)
        .await;

    // Create stream from receiver
    let rcs_id_for_stream = query.rcs_id.clone(); // 1. 克隆一份给流使用
    let stream = BroadcastStream::new(receiver)
        .filter_map(move |result| { // 2. 使用 move 捕获 rcs_id_for_stream
            let rcs_id = rcs_id_for_stream.clone(); // 3. 在异步块前再次克隆，供 async move 使用
            async move {
                match result {
                    Ok(event) => Some(Ok(event.to_sse_event())),
                    Err(e) => {
                        tracing::warn!("SSE receive error for RCS {}: {}", rcs_id, e);
                        Some(Err(e))
                    }
                }
            }
        });

    // Return SSE response with keep-alive
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive"),
    )
}

/// Push AGV arrived event to station
pub async fn notify_agv_arrived(
    state: &Arc<AppState>,
    station_id: &str,
    agv_id: &str,
) {
    let event = SseBroadcastEvent {
        target: SseTarget::Station(station_id.to_string()),
        event: SseEventType::AgvArrived {
            station_id: station_id.to_string(),
            agv_id: agv_id.to_string(),
        },
    };
    state.sse_manager.broadcast(event).await;
}

#[derive(Debug, Deserialize)]
struct TriggerAgvReq {
    station_id: String,
    agv_id: String,
}

async fn trigger_agv_arrived(
    State(state): State<Arc<AppState>>,
    Json(req): Json<TriggerAgvReq>,
) -> Json<serde_json::Value> {
    notify_agv_arrived(&state, &req.station_id, &req.agv_id).await;
    Json(serde_json::json!({
        "Code": 0,
        "Message": "Success",
        "Data": {
            "StationId": req.station_id,
            "AgvId": req.agv_id
        }
    }))
}

/// Push AGV left event to station
pub async fn notify_agv_left(
    state: &Arc<AppState>,
    station_id: &str,
    agv_id: &str,
) {
    let event = SseBroadcastEvent {
        target: SseTarget::Station(station_id.to_string()),
        event: SseEventType::AgvLeft {
            station_id: station_id.to_string(),
            agv_id: agv_id.to_string(),
        },
    };
    state.sse_manager.broadcast(event).await;
}

/// Push chute status change to RCS
pub async fn notify_chute_status_changed(
    state: &Arc<AppState>,
    platform_id: &str,
    grid_id: &str,
    status: &str,
) {
    let event = SseBroadcastEvent {
        target: SseTarget::Platform(platform_id.to_string()),
        event: SseEventType::ChuteStatusChanged {
            grid_id: grid_id.to_string(),
            status: status.to_string(),
        },
    };
    state.sse_manager.broadcast(event).await;
}
