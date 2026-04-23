//! RCS HTTP Client
//! SES调用RCS的HTTP客户端

use crate::core::error::{Result, SesError};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, error, info};

/// RCS Client configuration
#[derive(Debug, Clone)]
pub struct RcsClientConfig {
    pub base_url: String,
    pub timeout_ms: u64,
    pub retry_count: u32,
}

impl Default for RcsClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:12300".to_string(),
            timeout_ms: 10000,
            retry_count: 3,
        }
    }
}

/// RCS HTTP Client
#[derive(Debug, Clone)]
pub struct RcsClient {
    client: Client,
    config: RcsClientConfig,
}

/// Task dispatch request - 下发机器人任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsTaskDispatchReq {
    pub task_id: String,
    pub task_type: String, // DEPART_FLIP, DRIVER_EMPTY, etc.
    pub task_params: RcsTaskParams,
    pub create_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsTaskParams {
    pub order_id: Option<String>,
    pub agv_id: String,
    pub platform_id: String,
    pub box_code: Option<String>,
    pub rfid: Option<u32>,
    pub start_code: String,
    pub target_code: String,
}

/// Task cancel request - 取消任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsTaskCancelReq {
    pub task_id: String,
}

/// Grid switch request - 格口状态变更
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsChangeGridReq {
    pub model_id: String,
    pub switch_infos: Vec<GridSwitchInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridSwitchInfo {
    pub equipment_name: String,
    pub equipment_type: String, // STATION, CHUTE
    pub target_status: String,  // OPEN, CLOSE, FORBIDDEN
}

/// Wall type update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsUpdateWallTypeReq {
    pub mc_id: String,
    pub wall_type_id: String,
    pub chute_id: String,
    pub wall_location: String, // L, R
}

/// Wall state update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsUpdateWallStateReq {
    pub wall_rfid: u32,
    pub wall_state: u8, // 0:解除满包, 1:满包, 2:强制完成, 3:订单作业中被拔出
}

/// Generic RCS response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl RcsClient {
    /// Create new RCS client
    pub fn new(config: RcsClientConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_millis(config.timeout_ms))
            .build()
            .map_err(|e| SesError::Internal(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, config })
    }

    /// Create client with default config
    pub fn default_client() -> Result<Self> {
        Self::new(RcsClientConfig::default())
    }

    /// Update base URL
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.config.base_url = base_url;
        self
    }

    /// Dispatch task to AGV - 下发取放货任务
    /// POST /task/dispatch
    pub async fn dispatch_task(&self, req: RcsTaskDispatchReq) -> Result<RcsResponse<()>> {
        let url = format!("{}/task/dispatch", self.config.base_url);
        debug!("Dispatching task to RCS: {:?}", req);

        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| {
                error!("RCS dispatch_task request failed: {}", e);
                SesError::Internal(format!("RCS request failed: {}", e))
            })?;

        let status = response.status();
        let body = response
            .json::<RcsResponse<()>>()
            .await
            .map_err(|e| SesError::Internal(format!("Failed to parse RCS response: {}", e)))?;

        if status.is_success() && body.code == 0 {
            info!("Task {} dispatched successfully to RCS", req.task_id);
            Ok(body)
        } else {
            Err(SesError::Internal(format!(
                "RCS dispatch_task failed: code={}, msg={}",
                body.code, body.message
            )))
        }
    }

    /// Cancel task - 取消任务
    /// POST /task/cancel
    pub async fn cancel_task(&self, task_id: String) -> Result<RcsResponse<()>> {
        let url = format!("{}/task/cancel", self.config.base_url);
        let req = RcsTaskCancelReq { task_id };

        debug!("Cancelling task in RCS: {}", req.task_id);

        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| SesError::Internal(format!("RCS cancel_task failed: {}", e)))?;

        let body = response
            .json::<RcsResponse<()>>()
            .await
            .map_err(|e| SesError::Internal(format!("Failed to parse RCS response: {}", e)))?;

        Ok(body)
    }

    /// Batch switch grid status - 批量修改格口状态
    /// POST /model/equipment/batch/switch
    pub async fn batch_switch_grid(
        &self,
        model_id: String,
        switch_infos: Vec<GridSwitchInfo>,
    ) -> Result<RcsResponse<()>> {
        let url = format!("{}/model/equipment/batch/switch", self.config.base_url);
        let req = RcsChangeGridReq {
            model_id,
            switch_infos,
        };

        debug!("Batch switching grids in RCS: {:?}", req);

        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| SesError::Internal(format!("RCS batch_switch_grid failed: {}", e)))?;

        let body = response
            .json::<RcsResponse<()>>()
            .await
            .map_err(|e| SesError::Internal(format!("Failed to parse RCS response: {}", e)))?;

        Ok(body)
    }

    /// Switch single chute status - 修改单个格口状态
    pub async fn switch_chute(
        &self,
        model_id: String,
        chute_id: String,
        status: String,
    ) -> Result<RcsResponse<()>> {
        let switch_info = GridSwitchInfo {
            equipment_name: chute_id,
            equipment_type: "CHUTE".to_string(),
            target_status: status,
        };
        self.batch_switch_grid(model_id, vec![switch_info]).await
    }

    /// Switch single station status - 修改单个工作站状态
    pub async fn switch_station(
        &self,
        model_id: String,
        station_id: String,
        status: String,
    ) -> Result<RcsResponse<()>> {
        let switch_info = GridSwitchInfo {
            equipment_name: station_id,
            equipment_type: "STATION".to_string(),
            target_status: status,
        };
        self.batch_switch_grid(model_id, vec![switch_info]).await
    }

    /// Update wall type - 更新播种墙类型
    /// POST /update/wall/type
    pub async fn update_wall_type(
        &self,
        req: Vec<RcsUpdateWallTypeReq>,
    ) -> Result<RcsResponse<()>> {
        let url = format!("{}/update/wall/type", self.config.base_url);

        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| SesError::Internal(format!("RCS update_wall_type failed: {}", e)))?;

        let body = response
            .json::<RcsResponse<()>>()
            .await
            .map_err(|e| SesError::Internal(format!("Failed to parse RCS response: {}", e)))?;

        Ok(body)
    }

    /// Update wall state - 更新播种墙状态
    /// POST /update/wall/state
    pub async fn update_wall_state(&self, req: RcsUpdateWallStateReq) -> Result<RcsResponse<()>> {
        let url = format!("{}/update/wall/state", self.config.base_url);

        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| SesError::Internal(format!("RCS update_wall_state failed: {}", e)))?;

        let body = response
            .json::<RcsResponse<()>>()
            .await
            .map_err(|e| SesError::Internal(format!("Failed to parse RCS response: {}", e)))?;

        Ok(body)
    }

    /// Health check - 心跳检测
    /// POST /ping
    pub async fn ping(&self) -> Result<bool> {
        let url = format!("{}/ping", self.config.base_url);

        match self.client.post(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => {
                error!("RCS ping failed: {}", e);
                Ok(false)
            }
        }
    }

    /// Dispatch empty AGV - 驱离空车
    pub async fn dispatch_empty_agv(
        &self,
        agv_id: String,
        platform_id: String,
        station_id: String,
    ) -> Result<RcsResponse<()>> {
        let req = RcsTaskDispatchReq {
            task_id: format!("EMPTY_{}", uuid::Uuid::new_v4()),
            task_type: "DRIVER_EMPTY".to_string(),
            task_params: RcsTaskParams {
                order_id: None,
                agv_id,
                platform_id,
                box_code: None,
                rfid: None,
                start_code: station_id.clone(),
                target_code: station_id,
            },
            create_time: chrono::Utc::now().timestamp_millis(),
        };

        self.dispatch_task(req).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rcs_client_creation() {
        let client = RcsClient::default_client();
        assert!(client.is_ok());
    }
}
