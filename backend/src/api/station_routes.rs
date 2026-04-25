use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

use crate::core::state::AppState;
use crate::core::types::PageRequest;
use crate::core::error::{Result, SesError};
use crate::models::station::{
    BaseResult, LoginOutputDto, StationItemInfoVo, StationResponse, StationLoginRequest, StationLoginResponse,
    StationOperationRequest, StationTaskRequest, StationScanRequest,
    StationDispatchRequest, StationDepartRequest, TaskInfoData, TaskInfoResponse, VerifyNotifyRequest,
};

static VERIFIED_NOTIFY_IDS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn verify_notify_store() -> &'static Mutex<HashSet<String>> {
    VERIFIED_NOTIFY_IDS.get_or_init(|| Mutex::new(HashSet::new()))
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_stations))
        .route("/login", post(station_login))
        .route("/logout", post(station_logout))
        .route("/online", post(station_online))
        .route("/offline", post(station_offline))
        .route("/scan", post(station_scan))
        .route("/task", post(get_task))
        .route("/verifyNotify", post(verify_notify))
        .route("/distribute", post(dispatch_task))
        .route("/depart", post(robot_depart))
}

async fn list_stations(
    State(_state): State<Arc<AppState>>,
    Query(_params): Query<PageRequest>,
) -> Result<Json<Vec<StationResponse>>> {
    Ok(Json(vec![]))
}

async fn station_login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<StationLoginRequest>,
) -> Result<Json<StationLoginResponse>> {
    if request.username != "admin" || request.password != "123456" {
        return Err(SesError::Unauthorized);
    }

    let now = Utc::now();
    let exp = now + Duration::hours(state.config.jwt.expiration_hours);
    let claims = serde_json::json!({
        "sub": request.station_id,
        "platform_id": request.platform_id,
        "iat": now.timestamp(),
        "exp": exp.timestamp(),
    });

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt.secret.as_bytes()),
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

async fn station_logout(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<StationOperationRequest>,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"success": true})))
}

async fn station_online(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<StationOperationRequest>,
) -> Result<Json<StationResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn station_offline(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<StationOperationRequest>,
) -> Result<Json<StationResponse>> {
    Err(crate::core::error::SesError::NotFound("Not implemented".to_string()))
}

async fn station_scan(
    State(_state): State<Arc<AppState>>,
    Json(request): Json<StationScanRequest>,
) -> Result<Json<BaseResult<StationItemInfoVo>>> {
    let item = StationItemInfoVo {
        sku: request.barcode.clone(),
        sku_name: "联调商品".to_string(),
        barcode: request.barcode,
        image_url: None,
    };
    Ok(Json(BaseResult {
        code: 0,
        message: "Success".to_string(),
        data: Some(item),
    }))
}

async fn get_task(
    State(_state): State<Arc<AppState>>,
    Json(request): Json<StationTaskRequest>,
) -> Result<Json<TaskInfoResponse>> {
    let task = TaskInfoData {
        task_id: format!("T{}", Uuid::new_v4().simple()),
        chute_id: "CHUTE-05".to_string(),
        wave_id: "W1001".to_string(),
        order_id: format!("ORD-{}", request.barcode.unwrap_or_else(|| "UNKNOWN".to_string())),
        count: 1,
    };

    Ok(Json(TaskInfoResponse {
        code: 0,
        message: "Success".to_string(),
        data: Some(task),
    }))
}

async fn verify_notify(
    State(_state): State<Arc<AppState>>,
    Json(request): Json<VerifyNotifyRequest>,
) -> Result<Json<BaseResult<serde_json::Value>>> {
    let mut set = verify_notify_store()
        .lock()
        .map_err(|_| SesError::Internal("verifyNotify lock poisoned".to_string()))?;
    set.insert(request.sse_request_id.clone());

    Ok(Json(BaseResult {
        code: 0,
        message: "Success".to_string(),
        data: Some(serde_json::json!({
            "SseRequestId": request.sse_request_id,
            "Verified": true
        })),
    }))
}

async fn dispatch_task(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<StationDispatchRequest>,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"success": true})))
}

async fn robot_depart(
    State(state): State<Arc<AppState>>,
    Json(request): Json<StationDepartRequest>,
) -> Result<Json<BaseResult<serde_json::Value>>> {
    let departure_task = crate::clients::rcs_client::RcsTaskDispatchReq {
        task_id: request.task_id.clone(),
        task_type: "DEPART_FLIP".to_string(),
        task_params: crate::clients::rcs_client::RcsTaskParams {
            order_id: None,
            agv_id: request.agv_id.clone(),
            platform_id: request.platform_id.clone(),
            box_code: None,
            rfid: None,
            start_code: request.station_id.clone(),
            target_code: request.station_id.clone(),
        },
        create_time: Utc::now().timestamp_millis(),
    };

    if let Err(e) = state.rcs_client.dispatch_task(departure_task).await {
        tracing::warn!("RCS dispatch failed for robotDeparture: {}", e);
    }

    Ok(Json(BaseResult {
        code: 0,
        message: "Success".to_string(),
        data: Some(serde_json::json!({
            "TaskId": request.task_id,
            "AgvId": request.agv_id,
            "Completed": request.completed,
            "RequestId": request.request_id,
            "Accepted": true
        })),
    }))
}
