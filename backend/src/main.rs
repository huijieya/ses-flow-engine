use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

mod api;
mod clients;
mod core;
mod engine;
mod events;
mod models;
mod nodes;
mod devices;
mod sse;
mod utils;

use crate::clients::{RcsClient, RcsClientConfig};
use crate::core::{
    config::AppConfig,
    state::AppState,
};
use crate::engine::FlowEngine;
use crate::events::EventBus;
use crate::sse::SseManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ses_backend=info,tower_http=info".into()),
        )
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .init();

    info!("Starting SES Backend...");

    // Load configuration
    let config = AppConfig::load()?;
    info!("Configuration loaded successfully");

    // Initialize database connection pool
    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;

    info!("Database connection established");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await?;
    info!("Database migrations completed");

    // Initialize Redis connection
    let redis_client = redis::Client::open(config.redis.url.clone())?;
    let redis_conn = redis_client.get_multiplexed_async_connection().await?;
    info!("Redis connection established");

    // Initialize event bus
    let event_bus = EventBus::new(redis_conn.clone());
    info!("Event bus initialized");

    // Initialize flow engine
    let flow_engine = FlowEngine::new(db_pool.clone(), event_bus.clone());
    info!("Flow engine initialized");

    // Initialize RCS client
    let rcs_client_config = RcsClientConfig {
        base_url: config.rcs.url.clone(),
        timeout_ms: config.rcs.timeout_ms,
        retry_count: config.rcs.retry_count,
    };
    let rcs_client = RcsClient::new(rcs_client_config)?;
    info!("RCS client initialized");

    // Initialize SSE manager
    let sse_manager = SseManager::new();
    info!("SSE manager initialized");

    // Create application state
    let state = Arc::new(AppState {
        config,
        db_pool,
        redis_conn,
        event_bus,
        flow_engine,
        rcs_client,
        sse_manager,
    });

    // Build router
    let app = create_router(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        // API routes - using new modular structure
        .nest("/api/v1", api::create_routes())
        // Device callback endpoint
        .route("/ses/callback", post(api::callback_handler))
        // CORS
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any),
        )
        // State
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}
