use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// System status for home page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusVo {
    pub wave_day_po: WaveDayVo,
    pub twave_completed_compare_yest: i32,
    pub twave_completed: i32,
    pub tsorting_qty_compare_yest: i64,
    pub tsorting_qty: i64,
    pub tpack_qty_compare_yest: i64,
    pub twave_qty: i32,
    pub twave_qty_compare_yest: i32,
    pub tpack_qty: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveDayVo {
    pub twave_completed_compare_yest: i32,
    pub twave_completed: i32,
    pub twave_qty: i32,
    pub twave_qty_compare_yest: i32,
}

/// Sort efficiency for home page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortEfficiencyVo {
    pub times: Vec<String>,
    pub efficiencies: Vec<i32>,
    pub avg_efficiency: i32,
    pub high_efficiency: i32,
    pub low_efficiency: i32,
}

/// Home page response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomePageVo {
    pub system_status_vo: SystemStatusVo,
    pub sort_efficiency_vo: SortEfficiencyVo,
}

/// Base query for pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseQuery {
    pub page_num: i64,
    pub page_size: i64,
}

/// Page response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageVo<T> {
    pub total: i64,
    pub data: Vec<T>,
}

/// Wave execution info for home page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveExecVo {
    pub wave_id: String,
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_time: Option<chrono::DateTime<chrono::Utc>>,
    pub order_qty: i32,
    pub all_qty: i32,
    pub speed: f64,
    pub wave_status: String,
    pub wave_status_name: String,
    pub rate: i32,
}

/// Station sort info for home page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationSortVo {
    pub station_id: String,
    pub sort_qty: i32,
    pub efficiency: f64,
    pub station_status_name: String,
    pub sort_efficiency: i32,
    pub station_status: String,
}

/// Wave type info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveTypeVo {
    pub wave_type: i32,
    pub label: String,
}

/// Base response for SES 1.0 compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SesResponse<T> {
    pub msg: String,
    pub code: i32,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

impl<T> SesResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            msg: String::new(),
            code: 0,
            data,
            trace_id: Some(Uuid::new_v4().to_string()),
        }
    }

    pub fn error(msg: impl Into<String>) -> SesResponse<()> {
        SesResponse {
            msg: msg.into(),
            code: 1,
            data: (),
            trace_id: Some(Uuid::new_v4().to_string()),
        }
    }
}
