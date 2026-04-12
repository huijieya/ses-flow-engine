use crate::core::error::{Result, SesError};
use crate::devices::DeviceCapability;
use dashmap::DashMap;
use std::sync::Arc;

pub struct DeviceRegistry {
    capabilities: DashMap<String, Arc<dyn DeviceCapability>>,
}

impl DeviceRegistry {
    pub fn new() -> Self {
        Self {
            capabilities: DashMap::new(),
        }
    }

    pub fn register(&self, capability: Arc<dyn DeviceCapability>) {
        let device_type = capability.device_type().to_string();
        tracing::info!("Registering device capability: {}", device_type);
        self.capabilities.insert(device_type, capability);
    }

    pub fn get(&self, device_type: &str) -> Option<Arc<dyn DeviceCapability>> {
        self.capabilities.get(device_type).map(|c| c.clone())
    }

    pub fn has(&self, device_type: &str) -> bool {
        self.capabilities.contains_key(device_type)
    }

    pub fn list_capabilities(&self) -> Vec<String> {
        self.capabilities
            .iter()
            .map(|e| e.key().clone())
            .collect()
    }
}

impl Default for DeviceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
