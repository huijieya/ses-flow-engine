use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// RCS wall online request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsWallOnlineReq {
    pub mc_id: String,
    pub chute_id: String,
    pub rfid: i64,
    pub wall_location: String,
}

/// RCS wall online batch request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsWallOnlineBatchReq {
    pub data: Vec<RcsWallOnlineReq>,
}

/// RCS wall online response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsWallOnlineVo {
    pub wall_id: String,
    pub wall_type_id: String,
    pub rfid: i64,
    pub success: bool,
    pub reason: Option<String>,
}

/// RCS wall offline batch request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsWallOfflineBatchReq {
    pub data: Vec<i64>,
}

/// RCS wall offline response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsWallOfflineVo {
    pub wall_id: String,
    pub success: bool,
}

/// RCS update platform request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsUpdatePlatformReq {
    pub platform_id: String,
    pub platform_name: String,
    pub grids: Vec<RcsGridInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsGridInfo {
    pub grid_id: String,
    pub grid_name: String,
    pub grid_type: String,
    pub port_code: String,
    pub x: i32,
    pub y: i32,
}

/// RCS task status request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsTaskStatusReq {
    pub task_id: String,
    pub agv_id: String,
    pub result_code: String,
}

/// RCS robot flip request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsRobotFlipReq {
    pub task_id: String,
    pub target_id: String,
}

/// RCS robot flip response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsRobotFlipVo {
    pub chute_status: String,
}

/// RCS station AGV change request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsStationAgvChangeReq {
    pub agv_id: String,
    pub station_id: String,
}

/// Mini wall online request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniWallOnlineReq {
    pub mc_id: String,
    pub chute_id: String,
    pub rfid: i64,
    pub wall_location: String,
}

/// Mini wall offline request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniWallOfflineReq {
    pub rfid: i64,
}

/// Mini task status request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniTaskStatusReq {
    pub task_id: String,
    pub task_status: i32,
}

/// Mini item ready request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniItemReadyReq {
    pub station_id: String,
    pub exist: bool,
}

/// Mini AGV change request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniAgvChangeReq {
    pub station_id: String,
    pub agv_id: String,
}

/// Mini device status request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDeviceStatusReq {
    pub status: i32,
}

/// Wall online response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallOnlineVo {
    pub wall_id: String,
    pub wall_type_id: String,
    pub rfid: i64,
    pub success: bool,
    pub reason: Option<String>,
}
