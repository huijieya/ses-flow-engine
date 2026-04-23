//! SSE Manager - manages all SSE connections
//! SSE连接管理器

use super::{SseBroadcastEvent, SseEventType, SseTarget, StationConnection, RcsSseConnection};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, error, info, warn};

/// SSE connection manager
#[derive(Debug)]
pub struct SseManager {
    /// Station connections: station_id -> connection
    station_connections: RwLock<HashMap<String, Arc<StationConnection>>>,
    /// RCS connections: rcs_id -> connection
    rcs_connections: RwLock<HashMap<String, Arc<RcsSseConnection>>>,
    /// Platform to stations mapping
    platform_stations: RwLock<HashMap<String, Vec<String>>>,
    /// Global broadcast channel
    global_sender: broadcast::Sender<SseEventType>,
}

impl SseManager {
    /// Create new SSE manager
    pub fn new() -> Self {
        let (global_sender, _) = broadcast::channel(1000);
        Self {
            station_connections: RwLock::new(HashMap::new()),
            rcs_connections: RwLock::new(HashMap::new()),
            platform_stations: RwLock::new(HashMap::new()),
            global_sender,
        }
    }

    /// Register station SSE connection
    pub async fn register_station(
        &self,
        station_id: String,
        platform_id: String,
    ) -> broadcast::Receiver<SseEventType> {
        let (sender, receiver) = broadcast::channel(100);

        let connection = Arc::new(StationConnection {
            station_id: station_id.clone(),
            platform_id: platform_id.clone(),
            sender: sender.clone(),
            connected_at: chrono::Utc::now(),
        });

        // Store connection
        {
            let mut stations = self.station_connections.write().await;
            stations.insert(station_id.clone(), connection);
        }

        // Update platform mapping
        {
            let mut platforms = self.platform_stations.write().await;
            platforms
                .entry(platform_id.clone())
                .or_default()
                .push(station_id.clone());
        }

        info!(
            "Station {} registered for SSE on platform {}",
            station_id, platform_id
        );

        receiver
    }

    /// Unregister station SSE connection
    pub async fn unregister_station(&self, station_id: &str) {
        let platform_id = {
            let stations = self.station_connections.read().await;
            stations.get(station_id).map(|c| c.platform_id.clone())
        };

        if let Some(platform_id) = platform_id {
            // Remove from platform mapping
            let mut platforms = self.platform_stations.write().await;
            if let Some(stations) = platforms.get_mut(&platform_id) {
                stations.retain(|s| s != station_id);
            }
        }

        // Remove connection
        let mut stations = self.station_connections.write().await;
        stations.remove(station_id);

        info!("Station {} unregistered from SSE", station_id);
    }

    /// Register RCS SSE connection
    pub async fn register_rcs(
        &self,
        rcs_id: String,
        platform_ids: Vec<String>,
    ) -> broadcast::Receiver<SseEventType> {
        let (sender, receiver) = broadcast::channel(100);

        let connection = Arc::new(RcsSseConnection {
            rcs_id: rcs_id.clone(),
            platform_ids: platform_ids.clone(),
            sender: sender.clone(),
        });

        let mut rcs = self.rcs_connections.write().await;
        rcs.insert(rcs_id.clone(), connection);

        info!(
            "RCS {} registered for SSE with platforms {:?}",
            rcs_id, platform_ids
        );

        receiver
    }

    /// Unregister RCS SSE connection
    pub async fn unregister_rcs(&self, rcs_id: &str) {
        let mut rcs = self.rcs_connections.write().await;
        rcs.remove(rcs_id);
        info!("RCS {} unregistered from SSE", rcs_id);
    }

    /// Send event to specific station
    pub async fn send_to_station(&self, station_id: &str, event: SseEventType) {
        let stations = self.station_connections.read().await;
        if let Some(connection) = stations.get(station_id) {
            if let Err(e) = connection.sender.send(event) {
                warn!("Failed to send SSE to station {}: {}", station_id, e);
            } else {
                debug!("SSE sent to station {}", station_id);
            }
        } else {
            warn!("Station {} not connected for SSE", station_id);
        }
    }

    /// Send event to all stations on a platform
    pub async fn send_to_platform(&self, platform_id: &str, event: SseEventType) {
        let platforms = self.platform_stations.read().await;
        if let Some(stations) = platforms.get(platform_id) {
            for station_id in stations {
                self.send_to_station(station_id, event.clone()).await;
            }
        }
    }

    /// Send event to all stations
    pub async fn send_to_all_stations(&self, event: SseEventType) {
        let stations = self.station_connections.read().await;
        for (station_id, connection) in stations.iter() {
            if let Err(e) = connection.sender.send(event.clone()) {
                warn!("Failed to send SSE to station {}: {}", station_id, e);
            }
        }
    }

    /// Send event to RCS clients for a platform
    pub async fn send_to_rcs_platform(&self, platform_id: &str, event: SseEventType) {
        let rcs = self.rcs_connections.read().await;
        for (rcs_id, connection) in rcs.iter() {
            if connection.platform_ids.contains(&platform_id.to_string()) {
                if let Err(e) = connection.sender.send(event.clone()) {
                    warn!("Failed to send SSE to RCS {}: {}", rcs_id, e);
                } else {
                    debug!("SSE sent to RCS {} for platform {}", rcs_id, platform_id);
                }
            }
        }
    }

    /// Broadcast event based on target
    pub async fn broadcast(&self, event: SseBroadcastEvent) {
        match event.target {
            SseTarget::Station(station_id) => {
                self.send_to_station(&station_id, event.event).await;
            }
            SseTarget::Platform(platform_id) => {
                self.send_to_platform(&platform_id, event.event).await;
            }
            SseTarget::AllRcs => {
                // Send to all RCS connections
                let rcs = self.rcs_connections.read().await;
                for (rcs_id, connection) in rcs.iter() {
                    if let Err(e) = connection.sender.send(event.clone().event) {
                        warn!("Failed to send SSE to RCS {}: {}", rcs_id, e);
                    }
                }
            }
            SseTarget::AllStations => {
                self.send_to_all_stations(event.event).await;
            }
            SseTarget::Broadcast => {
                // Send to everyone
                self.send_to_all_stations(event.event.clone()).await;
                let rcs = self.rcs_connections.read().await;
                for (rcs_id, connection) in rcs.iter() {
                    if let Err(e) = connection.sender.send(event.clone().event) {
                        warn!("Failed to send SSE to RCS {}: {}", rcs_id, e);
                    }
                }
            }
        }
    }

    /// Get connected station count
    pub async fn station_count(&self) -> usize {
        self.station_connections.read().await.len()
    }

    /// Get connected RCS count
    pub async fn rcs_count(&self) -> usize {
        self.rcs_connections.read().await.len()
    }

    /// Check if station is connected
    pub async fn is_station_connected(&self, station_id: &str) -> bool {
        self.station_connections.read().await.contains_key(station_id)
    }
}

impl Default for SseManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sse_manager() {
        let manager = SseManager::new();
        
        // Register a station
        let _receiver = manager.register_station(
            "ST001".to_string(),
            "PL001".to_string(),
        ).await;

        assert_eq!(manager.station_count().await, 1);
        assert!(manager.is_station_connected("ST001").await);

        // Unregister
        manager.unregister_station("ST001").await;
        assert_eq!(manager.station_count().await, 0);
    }
}
