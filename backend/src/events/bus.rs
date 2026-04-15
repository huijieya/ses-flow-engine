use crate::core::error::{Result, SesError};
use crate::events::types::{Event, EventFilter, EventHandler, EventSubscription, EventType};
use dashmap::DashMap;
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Event bus for publish-subscribe messaging
#[derive(Clone)]
pub struct EventBus {
    redis: MultiplexedConnection,
    /// Local subscribers
    subscribers: Arc<DashMap<EventType, Vec<Uuid>>>,
    /// Broadcast channels for each event type
    channels: Arc<DashMap<EventType, broadcast::Sender<Event>>>,
}

impl EventBus {
    pub fn new(redis: MultiplexedConnection) -> Self {
        Self {
            redis,
            subscribers: Arc::new(DashMap::new()),
            channels: Arc::new(DashMap::new()),
        }
    }

    /// Publish an event
    pub async fn publish(&self, event: Event) -> Result<()> {
        info!("Publishing event: {:?} from {}", event.event_type, event.source);

        // Publish to local subscribers
        if let Some(tx) = self.channels.get(&event.event_type) {
            let _ = tx.send(event.clone());
        }

        // Publish to Redis for cross-instance distribution
        let event_json = serde_json::to_string(&event).map_err(|e| {
            SesError::EventBus(format!("Failed to serialize event: {}", e))
        })?;

        let channel = format!("ses:events:{}", event.event_type.as_str());
        let _: () = redis::cmd("PUBLISH")
            .arg(&channel)
            .arg(&event_json)
            .query_async(&mut self.redis.clone())
            .await
            .map_err(|e| SesError::EventBus(format!("Redis publish failed: {}", e)))?;

        Ok(())
    }

    /// Publish a simple event with custom type name and JSON payload.
    /// Convenience method for SES 1.0 compatibility endpoints.
    pub async fn publish_simple(
        &self,
        event_type_str: &str,
        payload: &serde_json::Value,
    ) -> Result<()> {
        let event = Event::new(
            EventType::Custom(event_type_str.to_string()),
            Uuid::nil(),
            "ses-api",
            payload,
        )
        .map_err(|e| SesError::EventBus(format!("Failed to create event: {}", e)))?;

        self.publish(event).await
    }

    /// Subscribe to events of specific types
    pub fn subscribe(&self, event_types: Vec<EventType>) -> broadcast::Receiver<Event> {
        let (tx, rx) = broadcast::channel(1000);

        for event_type in event_types {
            self.channels.entry(event_type).or_insert_with(|| tx.clone());
        }

        rx
    }

    /// Subscribe to a specific event type
    pub fn subscribe_to(&self, event_type: EventType) -> broadcast::Receiver<Event> {
        let (tx, rx) = broadcast::channel(1000);
        
        self.channels
            .entry(event_type)
            .and_modify(|existing| {
                // Merge receivers
            })
            .or_insert_with(|| tx);

        rx
    }

    /// Register an event handler
    pub async fn register_handler<H>(&self, handler: Arc<H>) -> Result<Uuid>
    where
        H: EventHandler + 'static,
    {
        let subscription_id = Uuid::new_v4();
        info!("Registering event handler: {}", subscription_id);

        // Store handler (simplified - in production use proper handler storage)
        // For now, we just use the broadcast mechanism

        Ok(subscription_id)
    }

    /// Unregister a handler
    pub fn unregister_handler(&self, subscription_id: Uuid) {
        info!("Unregistering event handler: {}", subscription_id);
        // Implementation would remove handler from storage
    }

    /// Wait for a specific event with timeout
    pub async fn wait_for_event(
        &self,
        event_type: EventType,
        filter: Option<EventFilter>,
        timeout_ms: u64,
    ) -> Result<Option<Event>> {
        let mut rx = self.subscribe_to(event_type);
        
        let deadline = tokio::time::Instant::now() + tokio::time::Duration::from_millis(timeout_ms);

        loop {
            let timeout = tokio::time::sleep_until(deadline);
            tokio::pin!(timeout);

            tokio::select! {
                Ok(event) = rx.recv() => {
                    if let Some(ref f) = filter {
                        if f.matches(&event) {
                            return Ok(Some(event));
                        }
                    } else {
                        return Ok(Some(event));
                    }
                }
                _ = &mut timeout => {
                    return Ok(None);
                }
            }
        }
    }

    /// Create a subscription for durable event processing
    pub async fn create_subscription(
        &self,
        name: impl Into<String>,
        event_types: Vec<EventType>,
        filter: Option<EventFilter>,
    ) -> Result<EventSubscription> {
        let subscription = EventSubscription {
            id: Uuid::new_v4(),
            event_types,
            handler: name.into(),
            filter,
        };

        // Persist subscription to Redis
        let sub_json = serde_json::to_string(&subscription).map_err(|e| {
            SesError::EventBus(format!("Failed to serialize subscription: {}", e))
        })?;

        let key = format!("ses:subscriptions:{}", subscription.id);
        let _: () = redis::cmd("SET")
            .arg(&key)
            .arg(&sub_json)
            .query_async(&mut self.redis.clone())
            .await
            .map_err(|e| SesError::EventBus(format!("Redis set failed: {}", e)))?;

        Ok(subscription)
    }

    /// Delete a subscription
    pub async fn delete_subscription(&self, subscription_id: Uuid) -> Result<()> {
        let key = format!("ses:subscriptions:{}", subscription_id);
        let _: () = redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut self.redis.clone())
            .await
            .map_err(|e| SesError::EventBus(format!("Redis del failed: {}", e)))?;

        Ok(())
    }

    /// Start listening for events from Redis (for distributed setup)
    pub async fn start_redis_listener(&self) -> Result<()> {
        // This would spawn a task that subscribes to Redis channels
        // and forwards events to local subscribers
        info!("Starting Redis event listener");
        Ok(())
    }

    /// Trigger a flow resume based on event
    pub async fn trigger_flow_resume(&self, instance_id: Uuid, event: Event) -> Result<()> {
        info!("Triggering flow resume for instance: {}", instance_id);

        // This would notify the flow engine to resume a paused flow instance
        // based on the received event

        Ok(())
    }
}

/// Event processor for handling events with backpressure
pub struct EventProcessor {
    bus: EventBus,
    handlers: Arc<DashMap<EventType, Vec<Arc<dyn EventHandler>>>>,
}

impl EventProcessor {
    pub fn new(bus: EventBus) -> Self {
        Self {
            bus,
            handlers: Arc::new(DashMap::new()),
        }
    }

    /// Add a handler for an event type
    pub fn add_handler(&self, event_type: EventType, handler: Arc<dyn EventHandler>) {
        self.handlers
            .entry(event_type)
            .or_default()
            .push(handler);
    }

    /// Start processing events
    pub async fn start(&self) -> Result<()> {
        // Subscribe to all event types that have handlers
        let event_types: Vec<EventType> = self.handlers.iter().map(|e| e.key().clone()).collect();
        
        if event_types.is_empty() {
            warn!("No event handlers registered");
            return Ok(());
        }

        let mut rx = self.bus.subscribe(event_types);

        info!("Event processor started");

        loop {
            match rx.recv().await {
                Ok(event) => {
                    if let Some(handlers) = self.handlers.get(&event.event_type) {
                        for handler in handlers.iter() {
                            if handler.can_handle(&event.event_type) {
                                if let Err(e) = handler.handle(&event).await {
                                    error!("Event handler failed: {}", e);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Event receiver error: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }
}
