use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{post, get},
    Json, Router,
};
use std::sync::Arc;

use crate::core::state::AppState;
use crate::models::pda::{
    PdaPackReq, PdaManDistributeReq, PdaBindBoxReq, StationTaskReq, StationTaskVo,
    ScanBarcodeReq, ScanBarcodeVo, StationOperationReq,
    StationInfoVo, SortModeReq, RequestDepartureReq, LockInventoryReq,
    ChuteOperationReq, ChuteOperationVo,
};
use crate::models::stats::SesResponse;

/// PDA and Station routes - external system integration
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        // PDA operations
        .route("/pda/pack", post(pda_pack))
        .route("/pda/manDistribute", post(pda_man_distribute))
        .route("/pda/bindBox", post(pda_bind_box))
        .route("/pda/bindBox/operateChute", post(pda_bind_box_operate_chute))
        .route("/pda/sse/connect/:pda_id", get(pda_sse_connect))
        // Station operations
        .route("/station/operation/getTaskInfo", post(station_get_task))
        .route("/station/operation/scanBarcode", post(station_scan_barcode))
        .route("/station/operation/requestOrder", post(station_request_order))
        .route("/station/operation/lockInventory", post(station_lock_inventory))
        .route("/station/operation/requestDeparture", post(station_request_departure))
        .route("/station/operation/chute/open", post(station_chute_open))
        .route("/station/operation/chute/close", post(station_chute_close))
        .route("/station/operation/returnToWork", post(station_return_to_work))
        .route("/station/operation/leaveWork", post(station_leave_work))
        .route("/station/operation/sortMode", post(station_sort_mode))
        .route("/station/operation/driveEmptyAgv", post(station_drive_empty_agv))
        .route("/station/operation/cancelLastAgv", post(station_cancel_last_agv))
        .route("/station/info", post(station_get_info))
        .route("/station/sse/connect/:station_id", get(station_sse_connect))
}

/// PDA pack operation
async fn pda_pack(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<PdaPackReq>,
) -> impl IntoResponse {
    tracing::info!("PDA pack: target_id={}", req.target_id);
    Json(SesResponse::success(()))
}

/// PDA manual distribute
async fn pda_man_distribute(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<PdaManDistributeReq>,
) -> impl IntoResponse {
    tracing::info!(
        "PDA manual distribute: task={}, sku={}, qty={}",
        req.task_id,
        req.sku,
        req.completed
    );
    Json(SesResponse::success(()))
}

/// PDA bind box
async fn pda_bind_box(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<PdaBindBoxReq>,
) -> impl IntoResponse {
    tracing::info!(
        "PDA bind box: target_id={}, container_id={}",
        req.target_id,
        req.container_id
    );
    Json(SesResponse::success(()))
}

/// PDA bind box and operate chute
async fn pda_bind_box_operate_chute(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<PdaBindBoxReq>,
) -> impl IntoResponse {
    tracing::info!(
        "PDA bind box & operate chute: target_id={}, container_id={}",
        req.target_id,
        req.container_id
    );
    Json(SesResponse::success(()))
}

/// PDA SSE connect (for real-time notifications)
async fn pda_sse_connect(
    Path(pda_id): Path<String>,
) -> impl IntoResponse {
    tracing::info!("PDA {} SSE connected", pda_id);
    Json(SesResponse::success(()))
}

// ============ Station Operations ============

/// Station get task info
async fn station_get_task(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<StationTaskReq>,
) -> impl IntoResponse {
    tracing::info!("Station {} get task, type={}", req.station_id, req.station_type);
    Json(SesResponse::success(None::<StationTaskVo>))
}

/// Station scan barcode
async fn station_scan_barcode(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<ScanBarcodeReq>,
) -> impl IntoResponse {
    tracing::info!("Station {} scan barcode: {}", req.station_id, req.barcode);
    Json(SesResponse::success(None::<ScanBarcodeVo>))
}

/// Station request order info
async fn station_request_order(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<StationTaskReq>,
) -> impl IntoResponse {
    tracing::info!("Station {} requesting order info", req.station_id);
    Json(SesResponse::success(()))
}

/// Station lock inventory
async fn station_lock_inventory(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<LockInventoryReq>,
) -> impl IntoResponse {
    tracing::info!(
        "Locking inventory: order={}, sku={}, qty={}",
        req.order_id,
        req.sku,
        req.qty
    );
    Json(SesResponse::success(()))
}

/// Station request departure
async fn station_request_departure(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<RequestDepartureReq>,
) -> impl IntoResponse {
    tracing::info!(
        "Station {} requesting departure for wave {} with {} orders",
        req.station_id,
        req.wave_id,
        req.order_ids.len()
    );
    Json(SesResponse::success(()))
}

/// Station chute open
async fn station_chute_open(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<ChuteOperationReq>,
) -> impl IntoResponse {
    tracing::info!("Opening chute: platform={}, chute={}", req.platform_id, req.chute_id);
    let response = ChuteOperationVo {
        success: true,
        chute_id: req.chute_id,
        status: "OPEN".to_string(),
    };
    Json(SesResponse::success(response))
}

/// Station chute close
async fn station_chute_close(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<ChuteOperationReq>,
) -> impl IntoResponse {
    tracing::info!("Closing chute: platform={}, chute={}", req.platform_id, req.chute_id);
    let response = ChuteOperationVo {
        success: true,
        chute_id: req.chute_id,
        status: "CLOSE".to_string(),
    };
    Json(SesResponse::success(response))
}

/// Station return to work
async fn station_return_to_work(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<StationOperationReq>,
) -> impl IntoResponse {
    tracing::info!("Station {} return to work", req.station_id);
    Json(SesResponse::success(()))
}

/// Station leave work
async fn station_leave_work(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<StationOperationReq>,
) -> impl IntoResponse {
    tracing::info!("Station {} leave work", req.station_id);
    Json(SesResponse::success(()))
}

/// Station sort mode selection
async fn station_sort_mode(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<SortModeReq>,
) -> impl IntoResponse {
    tracing::info!("Station {} sort mode: {}", req.station_id, req.mode);
    Json(SesResponse::success(()))
}

/// Station drive empty AGV
async fn station_drive_empty_agv(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<StationOperationReq>,
) -> impl IntoResponse {
    tracing::info!("Station {} driving empty AGV", req.station_id);
    Json(SesResponse::success(()))
}

/// Station cancel last AGV
async fn station_cancel_last_agv(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<StationOperationReq>,
) -> impl IntoResponse {
    tracing::info!("Station {} cancel last AGV", req.station_id);
    Json(SesResponse::success(()))
}

/// Get station info
async fn station_get_info(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<StationTaskReq>,
) -> impl IntoResponse {
    tracing::info!("Get station info: {}", req.station_id);
    Json(SesResponse::success(None::<StationInfoVo>))
}

/// Station SSE connect
async fn station_sse_connect(
    Path(station_id): Path<String>,
) -> impl IntoResponse {
    tracing::info!("Station {} SSE connected", station_id);
    Json(SesResponse::success(()))
}
