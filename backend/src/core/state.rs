use crate::core::config::AppConfig;
use crate::engine::FlowEngine;
use crate::events::EventBus;
use sqlx::PgPool;
use redis::aio::MultiplexedConnection;

pub struct AppState {
    pub config: AppConfig,
    pub db_pool: PgPool,
    pub redis_conn: MultiplexedConnection,
    pub event_bus: EventBus,
    pub flow_engine: FlowEngine,
}
