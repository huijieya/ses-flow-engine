use axum::{
    extract::State,
    response::IntoResponse,
    routing::{post, get},
    Json, Router,
};
use std::sync::Arc;

use crate::core::state::AppState;
use crate::models::rcs::{
    MiniAgvChangeReq, MiniDeviceStatusReq, MiniItemReadyReq, MiniTaskStatusReq,
    MiniWallOfflineReq, MiniWallOnlineReq, RcsRobotFlipReq, RcsRobotFlipVo,
    RcsStationAgvChangeReq, RcsTaskStatusReq, RcsUpdatePlatformReq,
    RcsWallOfflineBatchReq, RcsWallOnlineBatchReq, RcsWallOnlineVo, RcsWallOfflineVo,
    WallOnlineVo,
};
use crate::models::stats::SesResponse;

/// RCS routes - external system integration
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        // Wall management
        .route("/wall/batch/online", post(batch_wall_online))
        .route("/wall/batch/offline", post(batch_wall_offline))
        // RCS operations
        .route("/rcs/operation/updatePlatform", post(update_platform))
        .route("/rcs/operation/task/status", post(report_task_status))
        .route("/rcs/operation/robotRequestFlip", post(robot_request_flip))
        .route("/rcs/operation/leave/station", post(leave_station))
        .route("/rcs/operation/arrived/station", post(arrived_station))
        // Mini sorter operations
        .route("/mini/wall/online", post(mini_wall_online))
        .route("/mini/wall/offline", post(mini_wall_offline))
        .route("/mini/task/status", post(mini_task_status))
        .route("/mini/item/ready", post(mini_item_ready))
        .route("/mini/agv/change", post(mini_agv_change))
        .route("/mini/device/status", post(mini_device_status))
        .route("/mini/ping", post(mini_ping))
}

/// Batch wall online - RCS
async fn batch_wall_online(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<RcsWallOnlineBatchReq>,
) -> impl IntoResponse {
    let mut results = Vec::new();
    for wall_req in &req.data {
        results.push(RcsWallOnlineVo {
            wall_id: wall_req.mc_id.clone(),
            wall_type_id: format!("TYPE_{}", wall_req.wall_location),
            rfid: wall_req.rfid,
            success: true,
            reason: None,
        });
    }
    tracing::info!("Batch wall online: {} walls", req.data.len());
    Json(SesResponse::success(results))
}

/// Batch wall offline - RCS
async fn batch_wall_offline(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<RcsWallOfflineBatchReq>,
) -> impl IntoResponse {
    let mut results = Vec::new();
    for rfid in &req.data {
        results.push(RcsWallOfflineVo {
            wall_id: format!("WALL_{}", rfid),
            success: true,
        });
    }
    tracing::info!("Batch wall offline: {} walls", req.data.len());
    Json(SesResponse::success(results))
}

/// Update platform - RCS
async fn update_platform(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<Vec<RcsUpdatePlatformReq>>,
) -> impl IntoResponse {
    for platform in &req {
        tracing::info!("Updating platform: {}", platform.platform_id);
    }
    Json(SesResponse::success(()))
}

/// Report task status - RCS
async fn report_task_status(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<RcsTaskStatusReq>,
) -> impl IntoResponse {
    tracing::info!(
        "Task {} status update from AGV {}: {}",
        req.task_id,
        req.agv_id,
        req.result_code
    );
    Json(SesResponse::success(()))
}

/// Robot request flip - RCS
async fn robot_request_flip(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<RcsRobotFlipReq>,
) -> impl IntoResponse {
    tracing::info!("Robot request flip: task={}, target={}", req.task_id, req.target_id);
    let response = RcsRobotFlipVo {
        chute_status: "CLOSE".to_string(),
    };
    Json(SesResponse::success(response))
}

/// Leave station - RCS
async fn leave_station(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<RcsStationAgvChangeReq>,
) -> impl IntoResponse {
    tracing::info!("AGV {} left station {}", req.agv_id, req.station_id);
    Json(SesResponse::success(()))
}

/// Arrived station - RCS
async fn arrived_station(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<RcsStationAgvChangeReq>,
) -> impl IntoResponse {
    tracing::info!("AGV {} arrived at station {}", req.agv_id, req.station_id);
    Json(SesResponse::success(()))
}

// ============ Mini Sorter Operations ============

/// Mini wall online
async fn mini_wall_online(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<MiniWallOnlineReq>,
) -> impl IntoResponse {
    tracing::info!("Mini wall online: mc_id={}", req.mc_id);
    let response = WallOnlineVo {
        wall_id: req.mc_id.clone(),
        wall_type_id: format!("TYPE_{}", req.wall_location),
        rfid: req.rfid,
        success: true,
        reason: None,
    };
    Json(SesResponse::success(response))
}

/// Mini wall offline
async fn mini_wall_offline(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<MiniWallOfflineReq>,
) -> impl IntoResponse {
    tracing::info!("Mini wall offline: rfid={}", req.rfid);
    Json(SesResponse::success(()))
}

/// Mini task status
async fn mini_task_status(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<MiniTaskStatusReq>,
) -> impl IntoResponse {
    tracing::info!("Mini task status: task={}, status={}", req.task_id, req.task_status);
    Json(SesResponse::success(()))
}

/// Mini item ready
async fn mini_item_ready(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<MiniItemReadyReq>,
) -> impl IntoResponse {
    tracing::info!(
        "Item ready at station {}: exist={}",
        req.station_id,
        req.exist
    );
    Json(SesResponse::success(()))
}

/// Mini AGV change
async fn mini_agv_change(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<MiniAgvChangeReq>,
) -> impl IntoResponse {
    tracing::info!("Mini AGV change: station={}, agv={}", req.station_id, req.agv_id);
    Json(SesResponse::success(()))
}

/// Mini device status
async fn mini_device_status(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<MiniDeviceStatusReq>,
) -> impl IntoResponse {
    tracing::info!("Device status update: {}", req.status);
    Json(SesResponse::success(()))
}

/// Mini ping
async fn mini_ping() -> impl IntoResponse {
    Json(SesResponse::success(()))
}
