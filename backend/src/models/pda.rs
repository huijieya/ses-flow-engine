use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// PDA pack request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdaPackReq {
    pub target_id: String,
}

/// PDA manual distribute request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdaManDistributeReq {
    pub task_id: String,
    pub sku: String,
    pub barcode: String,
    pub wave_id: String,
    pub order_id: String,
    pub completed: i32,
    pub parcel_id: String,
    pub platform_id: String,
    pub wall_id: String,
    pub site_id: String,
}

/// PDA bind box request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdaBindBoxReq {
    pub target_id: String,
    pub container_id: String,
}

/// Station task info request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationTaskReq {
    pub station_id: String,
    pub station_type: String,
}

/// Station task info response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationTaskVo {
    pub task_id: String,
    pub task_type: String,
    pub wave_id: String,
    pub order_id: String,
    pub sku: String,
    pub barcode: String,
    pub qty: i32,
    pub completed_qty: i32,
    pub target_id: String,
    pub status: String,
}

/// Scan barcode request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanBarcodeReq {
    pub barcode: String,
    pub station_id: String,
}

/// Scan barcode response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanBarcodeVo {
    pub sku: String,
    pub sku_name: String,
    pub barcode: String,
    pub image_url: Option<String>,
    pub orders: Vec<SkuOrderInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkuOrderInfo {
    pub order_id: String,
    pub wave_id: String,
    pub qty: i32,
    pub completed_qty: i32,
}

/// Station operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationOperationReq {
    pub station_id: String,
    pub operation: String,
    pub params: Option<serde_json::Value>,
}

/// Station info response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationInfoVo {
    pub station_id: String,
    pub station_name: String,
    pub station_type: String,
    pub station_status: String,
    pub current_agv_id: Option<String>,
    pub current_wave_id: Option<String>,
}

/// Sort mode selection request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortModeReq {
    pub station_id: String,
    pub mode: String,
}

/// Request departure request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestDepartureReq {
    pub station_id: String,
    pub wave_id: String,
    pub order_ids: Vec<String>,
    pub agv_id: Option<String>,
    pub task_id: Option<String>,
    pub completed: Option<i32>,
}

/// Lock inventory request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockInventoryReq {
    pub order_id: String,
    pub sku: String,
    pub qty: i32,
}

/// Chute operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChuteOperationReq {
    pub platform_id: String,
    pub chute_id: String,
    pub operation: String,
}

/// Chute operation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChuteOperationVo {
    pub success: bool,
    pub chute_id: String,
    pub status: String,
}
