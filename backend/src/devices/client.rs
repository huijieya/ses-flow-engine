use crate::core::error::{Result, SesError};
use crate::devices::{DeviceCommand, DeviceResponse};
use reqwest::Client;
use std::time::Duration;

pub struct DeviceClient {
    client: Client,
    base_url: String,
}

impl DeviceClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: base_url.into(),
        }
    }

    pub async fn get_status(&self, device_id: &str) -> Result<DeviceResponse> {
        let url = format!("{}/device/{}/status", self.base_url, device_id);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| SesError::Device(format!("HTTP request failed: {}", e)))?;

        let status = response.status();
        if !status.is_success() {
            return Err(SesError::Device(format!("Device returned error: {}", status)));
        }

        let data = response.json().await.map_err(|e| {
            SesError::Device(format!("Failed to parse response: {}", e))
        })?;

        Ok(DeviceResponse {
            success: true,
            data: Some(data),
            error: None,
        })
    }

    pub async fn send_task(&self, device_id: &str, command: DeviceCommand) -> Result<DeviceResponse> {
        let url = format!("{}/device/{}/task", self.base_url, device_id);
        
        let response = self.client
            .post(&url)
            .json(&command)
            .send()
            .await
            .map_err(|e| SesError::Device(format!("HTTP request failed: {}", e)))?;

        let status = response.status();
        if !status.is_success() {
            return Err(SesError::Device(format!("Device returned error: {}", status)));
        }

        let data = response.json().await.map_err(|e| {
            SesError::Device(format!("Failed to parse response: {}", e))
        })?;

        Ok(DeviceResponse {
            success: true,
            data: Some(data),
            error: None,
        })
    }
}
