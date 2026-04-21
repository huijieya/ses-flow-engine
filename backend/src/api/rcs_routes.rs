use axum::{
    extract::State,
    response::IntoResponse,
    routing::{post, get},
    Json, Router,
};
use std::sync::Arc;

use crate::core::state::AppState;
use crate::core::error::SesError;
use crate::core::types::ExecutionContext;
use crate::models::rcs::{
    MiniAgvChangeReq, MiniDeviceStatusReq, MiniItemReadyReq, MiniTaskStatusReq,
    MiniWallOfflineReq, MiniWallOnlineReq, RcsRobotFlipReq, RcsRobotFlipVo,
    RcsStationAgvChangeReq, RcsTaskStatusReq, RcsUpdatePlatformReq,
    RcsWallOfflineBatchReq, RcsWallOnlineBatchReq, RcsWallOnlineVo, RcsWallOfflineVo,
    WallOnlineVo,
};
use crate::models::stats::SesResponse;

/// RCS routes - external system integration, internally routed through flow engine
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/check", get(check_health))
        .route("/wall/batch/online", post(batch_wall_online))
        .route("/wall/batch/offline", post(batch_wall_offline))
        .route("/rcs/operation/updatePlatform", post(update_platform))
        .route("/rcs/operation/task/status", post(report_task_status))
        .route("/rcs/operation/robotRequestFlip", post(robot_request_flip))
        .route("/rcs/operation/leave/station", post(leave_station))
        .route("/rcs/operation/arrived/station", post(arrived_station))
        .route("/mini/wall/online", post(mini_wall_online))
        .route("/mini/wall/offline", post(mini_wall_offline))
        .route("/mini/task/status", post(mini_task_status))
        .route("/mini/item/ready", post(mini_item_ready))
        .route("/mini/agv/change", post(mini_agv_change))
        .route("/mini/device/status", post(mini_device_status))
        .route("/mini/ping", post(mini_ping))
}

// check
async fn check_health() -> Result<impl IntoResponse, SesError> {
    Ok(Json(SesResponse:: success(())))
    }

/// Batch wall online - triggers wall_online node for each wall
async fn batch_wall_online(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RcsWallOnlineBatchReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut results = Vec::new();
    for wall_req in &req.data {
        // Trigger wall_online orchestration node
        let mut ctx = ExecutionContext::new();
        ctx.set("wall_id", &wall_req.mc_id);
        ctx.set("rfid", wall_req.rfid);

        let node_result = state.flow_engine
            .execute_node_direct("wall_online", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
            .await;

        let success = node_result.is_ok();
        let reason = node_result.err().map(|e| e.to_string());

        // Publish event for each wall
        let _ = state.event_bus.publish_simple("wall.online", &serde_json::json!({
            "wall_id": wall_req.mc_id,
            "rfid": wall_req.rfid,
            "success": success,
        })).await;

        results.push(RcsWallOnlineVo {
            wall_id: wall_req.mc_id.clone(),
            wall_type_id: format!("TYPE_{}", wall_req.wall_location),
            rfid: wall_req.rfid,
            success,
            reason,
        });
    }
    tracing::info!("Batch wall online: {} walls processed via flow engine", req.data.len());
    Ok(Json(SesResponse::success(results)))
}

/// Batch wall offline - triggers wall_offline node for each RFID
async fn batch_wall_offline(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RcsWallOfflineBatchReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut results = Vec::new();
    for rfid in &req.data {
        let mut ctx = ExecutionContext::new();
        ctx.set("rfid", rfid);

        let node_result = state.flow_engine
            .execute_node_direct("wall_offline", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
            .await;

        let success = node_result.is_ok();

        let _ = state.event_bus.publish_simple("wall.offline", &serde_json::json!({
            "rfid": rfid,
            "success": success,
        })).await;

        results.push(RcsWallOfflineVo {
            wall_id: format!("WALL_{}", rfid),
            success,
        });
    }
    tracing::info!("Batch wall offline: {} walls processed via flow engine", req.data.len());
    Ok(Json(SesResponse::success(results)))
}

/// Update platform - RCS
async fn update_platform(
    State(state): State<Arc<AppState>>,
    Json(req): Json<Vec<RcsUpdatePlatformReq>>,
) -> Result<impl IntoResponse, SesError> {
    for platform in &req {
        let mut ctx = ExecutionContext::new();
        ctx.set("platform_id", &platform.platform_id);
        let platform_name: &str = platform.platform_name.as_str();
        ctx.set("platform_name", platform_name);

        let _ = state.flow_engine
            .execute_node_direct("device_command", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
            .await;

        let _ = state.event_bus.publish_simple("rcs.platform_updated", &serde_json::json!({
            "platform_id": platform.platform_id,
        })).await;
    }
    tracing::info!("Updated {} platforms via flow engine", req.len());
    Ok(Json(SesResponse::success(())))
}

/// Report task status - RCS
async fn report_task_status(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RcsTaskStatusReq>,
) -> Result<impl IntoResponse, SesError> {
    tracing::info!("Task {} status from AGV {}: {}", req.task_id, req.agv_id, req.result_code);

    let _ = state.event_bus.publish_simple("rcs.task_status", &serde_json::json!({
        "task_id": req.task_id,
        "agv_id": req.agv_id,
        "result_code": req.result_code,
    })).await;

    Ok(Json(SesResponse::success(())))
}

/// Robot request flip - triggers rcs_dispatch node
async fn robot_request_flip(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RcsRobotFlipReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("agv_id", &req.task_id);
    ctx.set("target_station", &req.target_id);
    ctx.set("task_type", "flip");

    let node_result = state.flow_engine
        .execute_node_direct("rcs_dispatch", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await;

    match node_result {
        Ok(result) => {
            tracing::info!("Robot flip dispatched via flow engine: {:?}", result.output);
            let chute_status = result.output.get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("CLOSE");
            Ok(Json(SesResponse::success(RcsRobotFlipVo {
                chute_status: chute_status.to_string(),
            })))
        }
        Err(e) => {
            tracing::warn!("Flow engine rcs_dispatch failed: {}", e);
            Ok(Json(SesResponse::success(RcsRobotFlipVo {
                chute_status: "CLOSE".to_string(),
            })))
        }
    }
}

/// Leave station - triggers rcs dispatch event
async fn leave_station(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RcsStationAgvChangeReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("agv_id", &req.agv_id);
    ctx.set("station_id", &req.station_id);

    let _ = state.flow_engine
        .execute_node_direct("rcs_arrived", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await;

    let _ = state.event_bus.publish_simple("rcs.agv_left", &serde_json::json!({
        "agv_id": req.agv_id,
        "station_id": req.station_id,
    })).await;

    tracing::info!("AGV {} left station {}", req.agv_id, req.station_id);
    Ok(Json(SesResponse::success(())))
}

/// Arrived station - triggers rcs_arrived node and notify station via SSE
async fn arrived_station(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RcsStationAgvChangeReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("agv_id", &req.agv_id);
    ctx.set("station_id", &req.station_id);

    match state.flow_engine
        .execute_node_direct("rcs_arrived", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await
    {
        Ok(result) => {
            tracing::info!("AGV {} arrived at station {} via flow engine: {:?}", req.agv_id, req.station_id, result.output);
        }
        Err(e) => {
            tracing::warn!("Flow engine rcs_arrived failed: {}", e);
        }
    }

    // Publish event
    let _ = state.event_bus.publish_simple("rcs.agv_arrived", &serde_json::json!({
        "agv_id": req.agv_id,
        "station_id": req.station_id,
    })).await;

    // Notify station via SSE
    crate::api::sse_routes::notify_agv_arrived(
        &state,
        &req.station_id,
        &req.agv_id,
    ).await;

    Ok(Json(SesResponse::success(())))
}

// ============ Mini Sorter Operations ============

/// Mini wall online - triggers wall_online node
async fn mini_wall_online(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MiniWallOnlineReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("wall_id", &req.mc_id);
    ctx.set("rfid", req.rfid);

    let success = match state.flow_engine
        .execute_node_direct("wall_online", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await
    {
        Ok(result) => {
            tracing::info!("Mini wall online via flow engine: {:?}", result.output);
            true
        }
        Err(e) => {
            tracing::warn!("Flow engine wall_online failed: {}", e);
            false
        }
    };

    let _ = state.event_bus.publish_simple("mini.wall_online", &serde_json::json!({
        "mc_id": req.mc_id,
        "rfid": req.rfid,
    })).await;

    Ok(Json(SesResponse::success(WallOnlineVo {
        wall_id: req.mc_id.clone(),
        wall_type_id: format!("TYPE_{}", req.wall_location),
        rfid: req.rfid,
        success,
        reason: if success { None } else { Some("Flow engine error".to_string()) },
    })))
}

/// Mini wall offline - triggers wall_offline node
async fn mini_wall_offline(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MiniWallOfflineReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("rfid", req.rfid);

    let _ = state.flow_engine
        .execute_node_direct("wall_offline", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await;

    let _ = state.event_bus.publish_simple("mini.wall_offline", &serde_json::json!({
        "rfid": req.rfid,
    })).await;

    tracing::info!("Mini wall offline: rfid={}", req.rfid);
    Ok(Json(SesResponse::success(())))
}

/// Mini task status
async fn mini_task_status(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MiniTaskStatusReq>,
) -> Result<impl IntoResponse, SesError> {
    let _ = state.event_bus.publish_simple("mini.task_status", &serde_json::json!({
        "task_id": req.task_id,
        "task_status": req.task_status,
    })).await;

    tracing::info!("Mini task status: task={}, status={}", req.task_id, req.task_status);
    Ok(Json(SesResponse::success(())))
}

/// Mini item ready - triggers station_get_task node
async fn mini_item_ready(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MiniItemReadyReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("station_id", &req.station_id);

    let _ = state.flow_engine
        .execute_node_direct("station_get_task", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await;

    let _ = state.event_bus.publish_simple("mini.item_ready", &serde_json::json!({
        "station_id": req.station_id,
        "exist": req.exist,
    })).await;

    tracing::info!("Item ready at station {}: exist={}", req.station_id, req.exist);
    Ok(Json(SesResponse::success(())))
}

/// Mini AGV change - triggers rcs_arrived node
async fn mini_agv_change(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MiniAgvChangeReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("agv_id", &req.agv_id);
    ctx.set("station_id", &req.station_id);

    let _ = state.flow_engine
        .execute_node_direct("rcs_arrived", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await;

    let _ = state.event_bus.publish_simple("mini.agv_change", &serde_json::json!({
        "station_id": req.station_id,
        "agv_id": req.agv_id,
    })).await;

    tracing::info!("Mini AGV change: station={}, agv={}", req.station_id, req.agv_id);
    Ok(Json(SesResponse::success(())))
}

/// Mini device status
async fn mini_device_status(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MiniDeviceStatusReq>,
) -> Result<impl IntoResponse, SesError> {
    let _ = state.event_bus.publish_simple("mini.device_status", &serde_json::json!({
        "status": req.status,
    })).await;

    tracing::info!("Device status update: {}", req.status);
    Ok(Json(SesResponse::success(())))
}

/// Mini ping
async fn mini_ping() -> Result<impl IntoResponse, SesError> {
    Ok(Json(SesResponse::success(())))
}
