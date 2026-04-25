use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{post, get},
    Json, Router,
};
use std::sync::Arc;

use crate::core::state::AppState;
use crate::core::error::SesError;
use crate::core::types::ExecutionContext;
use crate::models::pda::{
    PdaPackReq, PdaManDistributeReq, PdaBindBoxReq, StationTaskReq, StationTaskVo,
    ScanBarcodeReq, ScanBarcodeVo, StationOperationReq,
    StationInfoVo, SortModeReq, RequestDepartureReq, LockInventoryReq,
    ChuteOperationReq, ChuteOperationVo,
};
use crate::models::station::{
    BaseResult, LoginOutputDto, StationDepartRequest, StationLoginRequest, StationLoginResponse,
    VerifyNotifyRequest,
};
use crate::models::stats::SesResponse;

/// PDA and Station routes - external integration, internally routed through flow engine
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/pda/pack", post(pda_pack))
        .route("/pda/manDistribute", post(pda_man_distribute))
        .route("/pda/bindBox", post(pda_bind_box))
        .route("/pda/bindBox/operateChute", post(pda_bind_box_operate_chute))
        .route("/pda/sse/connect/:pda_id", get(pda_sse_connect))
        .route("/station/operation/getTaskInfo", post(station_get_task))
        .route("/station/operation/scanBarcode", post(station_scan_barcode))
        .route("/station/operation/login", post(station_login))
        .route("/station/operation/verifyNotify", post(station_verify_notify))
        .route("/station/operation/robotDeparture", post(station_robot_departure))
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

async fn station_login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StationLoginRequest>,
) -> Result<impl IntoResponse, SesError> {
    if req.username != "admin" || req.password != "123456" {
        return Err(SesError::Unauthorized);
    }
    let now = chrono::Utc::now();
    let exp = now + chrono::Duration::hours(state.config.jwt.expiration_hours);
    let claims = serde_json::json!({
        "sub": req.station_id,
        "platform_id": req.platform_id,
        "iat": now.timestamp(),
        "exp": exp.timestamp(),
    });
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(state.config.jwt.secret.as_bytes()),
    )
    .map_err(|e| SesError::Internal(format!("JWT encode failed: {}", e)))?;

    Ok(Json(StationLoginResponse {
        code: 0,
        message: "Success".to_string(),
        data: Some(LoginOutputDto {
            authorization: format!("Bearer {}", token),
        }),
    }))
}

async fn station_verify_notify(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<VerifyNotifyRequest>,
) -> Result<impl IntoResponse, SesError> {
    tracing::info!("verifyNotify confirmed: {}", req.sse_request_id);
    Ok(Json(BaseResult {
        code: 0,
        message: "Success".to_string(),
        data: Some(serde_json::json!({
            "SseRequestId": req.sse_request_id,
            "Verified": true
        })),
    }))
}

async fn station_robot_departure(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StationDepartRequest>,
) -> Result<impl IntoResponse, SesError> {
    let rcs_req = crate::clients::rcs_client::RcsTaskDispatchReq {
        task_id: req.task_id.clone(),
        task_type: "DEPART_FLIP".to_string(),
        task_params: crate::clients::rcs_client::RcsTaskParams {
            order_id: None,
            agv_id: req.agv_id.clone(),
            platform_id: req.platform_id.clone(),
            box_code: None,
            rfid: None,
            start_code: req.station_id.clone(),
            target_code: req.station_id.clone(),
        },
        create_time: chrono::Utc::now().timestamp_millis(),
    };
    if let Err(e) = state.rcs_client.dispatch_task(rcs_req).await {
        tracing::warn!("robotDeparture RCS dispatch failed: {}", e);
    }

    Ok(Json(BaseResult {
        code: 0,
        message: "Success".to_string(),
        data: Some(serde_json::json!({
            "TaskId": req.task_id,
            "AgvId": req.agv_id,
            "Completed": req.completed,
            "RequestId": req.request_id
        })),
    }))
}

/// PDA pack - triggers pack_operation node
async fn pda_pack(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PdaPackReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("target_id", &req.target_id);

    match state.flow_engine
        .execute_node_direct("pack_operation", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await
    {
        Ok(result) => {
            tracing::info!("PDA pack via flow engine: {:?}", result.output);
        }
        Err(e) => {
            tracing::warn!("Flow engine pack_operation failed: {}", e);
        }
    }

    let _ = state.event_bus.publish_simple("pda.pack", &serde_json::json!({
        "target_id": req.target_id,
    })).await;

    Ok(Json(SesResponse::success(())))
}

/// PDA manual distribute - triggers distribute_item node
async fn pda_man_distribute(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PdaManDistributeReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("target_id", &req.task_id);
    ctx.set("sku", &req.sku);
    ctx.set("quantity", req.completed);

    match state.flow_engine
        .execute_node_direct("distribute_item", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await
    {
        Ok(result) => {
            tracing::info!("PDA distribute via flow engine: {:?}", result.output);
        }
        Err(e) => {
            tracing::warn!("Flow engine distribute_item failed: {}", e);
        }
    }

    let _ = state.event_bus.publish_simple("pda.man_distribute", &serde_json::json!({
        "task_id": req.task_id,
        "sku": req.sku,
        "completed": req.completed,
    })).await;

    Ok(Json(SesResponse::success(())))
}

/// PDA bind box - triggers pack_operation + chute_operation nodes
async fn pda_bind_box(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PdaBindBoxReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("target_id", &req.target_id);
    ctx.set("box_code", &req.container_id);

    let _ = state.flow_engine
        .execute_node_direct("pack_operation", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await;

    let _ = state.event_bus.publish_simple("pda.bind_box", &serde_json::json!({
        "target_id": req.target_id,
        "container_id": req.container_id,
    })).await;

    tracing::info!("PDA bind box: target={}, container={}", req.target_id, req.container_id);
    Ok(Json(SesResponse::success(())))
}

/// PDA bind box and operate chute - triggers pack + chute nodes
async fn pda_bind_box_operate_chute(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PdaBindBoxReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("target_id", &req.target_id);
    ctx.set("box_code", &req.container_id);

    // Step 1: Bind box (pack)
    let _ = state.flow_engine
        .execute_node_direct("pack_operation", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await;

    // Step 2: Open chute for the target
    let mut chute_ctx = ExecutionContext::new();
    chute_ctx.set("chute_id", &req.target_id);
    chute_ctx.set("operation", "open");
    let _ = state.flow_engine
        .execute_node_direct("chute_operation", serde_json::to_value(&chute_ctx).unwrap_or_default(), &mut chute_ctx)
        .await;

    tracing::info!("PDA bind box & operate chute: target={}, container={}", req.target_id, req.container_id);
    Ok(Json(SesResponse::success(())))
}

/// PDA SSE connect
async fn pda_sse_connect(
    Path(pda_id): Path<String>,
) -> Result<impl IntoResponse, SesError> {
    tracing::info!("PDA {} SSE connected", pda_id);
    Ok(Json(SesResponse::success(())))
}

// ============ Station Operations ============

/// Station get task - query real tasks from database
async fn station_get_task(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StationTaskReq>,
) -> Result<impl IntoResponse, SesError> {
    // Query active waves for this station's platform
    let tasks: Vec<TaskInfoVo> = sqlx::query_as(
        r#"
        SELECT 
            o.order_id as task_id,
            'pick' as task_type,
            b.chute_id as target_location,
            od.sku,
            (od.qty - od.completed_qty) as quantity,
            o.wave_id
        FROM ses_orders o
        JOIN ses_waves w ON o.wave_id = w.wave_id
        JOIN ses_order_details od ON o.order_id = od.order_id
        LEFT JOIN ses_order_chute_bindings b ON o.order_id = b.order_id AND b.status = 'ACTIVE'
        JOIN ses_stations s ON s.wave_id = o.wave_id
        WHERE s.station_id = $1
            AND w.status = 'STARTED'
            AND o.status IN ('STARTED', 'IN_PROGRESS')
            AND (od.qty - od.completed_qty) > 0
        ORDER BY o.priority DESC, o.created_at ASC
        LIMIT 10
        "#,
    )
    .bind(&req.station_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(SesError::Database)?;

    if tasks.is_empty() {
        return Ok(Json(SesResponse::success(serde_json::json!({
            "has_task": false,
            "station_id": req.station_id
        }))));
    }

    let has_task = !tasks.is_empty();
    let response = serde_json::json!({
        "has_task": has_task,
        "station_id": req.station_id,
        "tasks": tasks,
        "task_count": tasks.len()
    });

    tracing::info!("Station {} get {} tasks", req.station_id, tasks.len());
    Ok(Json(SesResponse::success(response)))
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
struct TaskInfoVo {
    task_id: String,
    task_type: String,
    target_location: Option<String>,
    sku: String,
    quantity: i32,
    wave_id: String,
}

/// Station scan barcode - triggers station_scan node
async fn station_scan_barcode(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ScanBarcodeReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("station_id", &req.station_id);
    ctx.set("barcode", &req.barcode);

    match state.flow_engine
        .execute_node_direct("station_scan", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await
    {
        Ok(result) => {
            tracing::info!("Station scan via flow engine: {:?}", result.output);
            let verified = result.output.get("verified")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            Ok(Json(SesResponse::success(ScanBarcodeVo {
                sku: req.barcode.clone(),
                sku_name: if verified { "验证成功".to_string() } else { "验证失败".to_string() },
                barcode: req.barcode.clone(),
                image_url: None,
                orders: vec![],
            })))
        }
        Err(e) => {
            tracing::warn!("Flow engine station_scan failed: {}", e);
            Ok(Json(SesResponse::success(ScanBarcodeVo {
                sku: req.barcode.clone(),
                sku_name: "扫描失败".to_string(),
                barcode: req.barcode.clone(),
                image_url: None,
                orders: vec![],
            })))
        }
    }
}

/// Station request order info
async fn station_request_order(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StationTaskReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("station_id", &req.station_id);

    let _ = state.flow_engine
        .execute_node_direct("station_get_task", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await;

    let _ = state.event_bus.publish_simple("station.request_order", &serde_json::json!({
        "station_id": req.station_id,
    })).await;

    tracing::info!("Station {} requesting order info", req.station_id);
    Ok(Json(SesResponse::success(())))
}

/// Station lock inventory
async fn station_lock_inventory(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LockInventoryReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("order_id", &req.order_id);
    ctx.set("sku", &req.sku);
    ctx.set("quantity", req.qty);

    let _ = state.flow_engine
        .execute_node_direct("order_assign", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await;

    let _ = state.event_bus.publish_simple("station.lock_inventory", &serde_json::json!({
        "order_id": req.order_id,
        "sku": req.sku,
        "qty": req.qty,
    })).await;

    tracing::info!("Lock inventory: order={}, sku={}, qty={}", req.order_id, req.sku, req.qty);
    Ok(Json(SesResponse::success(())))
}

/// Station request departure - triggers task_dispatch + rcs_dispatch flow
async fn station_request_departure(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RequestDepartureReq>,
) -> Result<impl IntoResponse, SesError> {
    // Get station info for AGV
    let agv_id = req.agv_id.clone().unwrap_or_else(|| "UNKNOWN".to_string());
    
    // For each order, dispatch task (assign destination and send to RCS)
    for order_id in &req.order_ids {
        // Step 1: Assign destination and deduct quantity
        let mut dispatch_ctx = ExecutionContext::new();
        dispatch_ctx.set("order_id", order_id);
        dispatch_ctx.set("station_id", &req.station_id);
        dispatch_ctx.set("agv_id", &agv_id);
        
        match state.flow_engine
            .execute_node_direct("task_dispatch", serde_json::to_value(&dispatch_ctx).unwrap_or_default(), &mut dispatch_ctx)
            .await
        {
            Ok(result) => {
                tracing::info!("Task dispatched for order {}: {:?}", order_id, result.output);
                
                // Get assigned chute from result
                let chute_id = result.output.get("chute_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                // Step 2: Send task to RCS
                let platform_id = result.output.get("platform_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                let rcs_req = crate::clients::rcs_client::RcsTaskDispatchReq {
                    task_id: format!("TASK_{}", uuid::Uuid::new_v4()),
                    task_type: "DEPART_FLIP".to_string(),
                    task_params: crate::clients::rcs_client::RcsTaskParams {
                        order_id: Some(order_id.clone()),
                        agv_id: agv_id.clone(),
                        platform_id: platform_id.to_string(),
                        box_code: None,
                        rfid: None,
                        start_code: req.station_id.clone(),
                        target_code: chute_id.to_string(),
                    },
                    create_time: chrono::Utc::now().timestamp_millis(),
                };
                
                // Call RCS to dispatch task
                match state.rcs_client.dispatch_task(rcs_req).await {
                    Ok(_) => tracing::info!("RCS task dispatched for order {}", order_id),
                    Err(e) => tracing::warn!("Failed to dispatch RCS task for order {}: {}", order_id, e),
                }
            }
            Err(e) => {
                tracing::warn!("Task dispatch failed for order {}: {}", order_id, e);
            }
        }
    }

    let _ = state.event_bus.publish_simple("station.request_departure", &serde_json::json!({
        "station_id": req.station_id,
        "wave_id": req.wave_id,
        "order_count": req.order_ids.len(),
    })).await;

    tracing::info!("Station {} departure: wave={}, {} orders", req.station_id, req.wave_id, req.order_ids.len());
    Ok(Json(SesResponse::success(serde_json::json!({
        "success": true,
        "dispatched_orders": req.order_ids.len()
    }))))
}

/// Station chute open - triggers chute_operation node
async fn station_chute_open(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChuteOperationReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("chute_id", &req.chute_id);
    ctx.set("operation", "open");

    match state.flow_engine
        .execute_node_direct("chute_operation", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await
    {
        Ok(result) => {
            tracing::info!("Chute open via flow engine: {:?}", result.output);
        }
        Err(e) => {
            tracing::warn!("Flow engine chute_operation failed: {}", e);
        }
    }

    Ok(Json(SesResponse::success(ChuteOperationVo {
        success: true,
        chute_id: req.chute_id,
        status: "OPEN".to_string(),
    })))
}

/// Station chute close - triggers chute_operation node
async fn station_chute_close(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChuteOperationReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("chute_id", &req.chute_id);
    ctx.set("operation", "close");

    match state.flow_engine
        .execute_node_direct("chute_operation", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await
    {
        Ok(result) => {
            tracing::info!("Chute close via flow engine: {:?}", result.output);
        }
        Err(e) => {
            tracing::warn!("Flow engine chute_operation failed: {}", e);
        }
    }

    Ok(Json(SesResponse::success(ChuteOperationVo {
        success: true,
        chute_id: req.chute_id,
        status: "CLOSE".to_string(),
    })))
}

/// Station return to work
async fn station_return_to_work(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StationOperationReq>,
) -> Result<impl IntoResponse, SesError> {
    let _ = state.event_bus.publish_simple("station.return_to_work", &serde_json::json!({
        "station_id": req.station_id,
    })).await;

    tracing::info!("Station {} return to work", req.station_id);
    Ok(Json(SesResponse::success(())))
}

/// Station leave work
async fn station_leave_work(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StationOperationReq>,
) -> Result<impl IntoResponse, SesError> {
    let _ = state.event_bus.publish_simple("station.leave_work", &serde_json::json!({
        "station_id": req.station_id,
    })).await;

    tracing::info!("Station {} leave work", req.station_id);
    Ok(Json(SesResponse::success(())))
}

/// Station sort mode
async fn station_sort_mode(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SortModeReq>,
) -> Result<impl IntoResponse, SesError> {
    let _ = state.event_bus.publish_simple("station.sort_mode", &serde_json::json!({
        "station_id": req.station_id,
        "mode": req.mode,
    })).await;

    tracing::info!("Station {} sort mode: {}", req.station_id, req.mode);
    Ok(Json(SesResponse::success(())))
}

/// Station drive empty AGV - triggers rcs_dispatch node
async fn station_drive_empty_agv(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StationOperationReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("target_station", &req.station_id);
    ctx.set("task_type", "drive_empty");

    let _ = state.flow_engine
        .execute_node_direct("rcs_dispatch", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await;

    tracing::info!("Station {} driving empty AGV", req.station_id);
    Ok(Json(SesResponse::success(())))
}

/// Station cancel last AGV
async fn station_cancel_last_agv(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StationOperationReq>,
) -> Result<impl IntoResponse, SesError> {
    let _ = state.event_bus.publish_simple("rcs.cancel_agv", &serde_json::json!({
        "station_id": req.station_id,
    })).await;

    tracing::info!("Station {} cancel last AGV", req.station_id);
    Ok(Json(SesResponse::success(())))
}

/// Get station info
async fn station_get_info(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StationTaskReq>,
) -> Result<impl IntoResponse, SesError> {
    let mut ctx = ExecutionContext::new();
    ctx.set("station_id", &req.station_id);

    match state.flow_engine
        .execute_node_direct("station_get_task", serde_json::to_value(&ctx).unwrap_or_default(), &mut ctx)
        .await
    {
        Ok(result) => {
            tracing::info!("Station info via flow engine: {:?}", result.output);
            Ok(Json(SesResponse::success(result.output)))
        }
        Err(e) => {
            tracing::warn!("Flow engine station_get_task failed: {}", e);
            Ok(Json(SesResponse::success(serde_json::json!({"station_id": req.station_id}))))
        }
    }
}

/// Station SSE connect
async fn station_sse_connect(
    Path(station_id): Path<String>,
) -> Result<impl IntoResponse, SesError> {
    tracing::info!("Station {} SSE connected", station_id);
    Ok(Json(SesResponse::success(())))
}
