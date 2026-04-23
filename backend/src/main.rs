use axum::{
    routing::{get, post},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
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
    // let app = create_router(state);

    // Start server
    // let http_addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 8080));
    // info!("Server starting on http://{}", http_addr);

    // let listener = tokio::net::TcpListener::bind(http_addr).await?;
    // axum::serve(listener, app).await?;
    
    // HTTPS配置
    // let cert_path = "../certs/cert.pem";
    // let key_path = "../certs/key.pem";

    // let tls_config = RustlsConfig::from_pem_file(cert_path, key_path)
    //     .await
    //     .map_err(|e| anyhow::anyhow!("Failed to load TLS config: {}", e))?;

    // let https_addr = SocketAddr::from(([0, 0, 0, 0], 8443));
    // info!("Server starting on https://{}", https_addr);
    // axum_server::bind_rustls(https_addr, tls_config)
    //     .serve(app.into_make_service())
    //     .await?;

    //-----
        let app = create_router(state);

    // 1. HTTP Server for localhost debugging
    let http_addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let http_app = app.clone(); // Router implements Clone
    
    tokio::spawn(async move {
        info!("HTTP Server starting on http://{}", http_addr);
        axum::serve(tokio::net::TcpListener::bind(http_addr).await.unwrap(), http_app)
            .await
            .unwrap();
    });

    // 2. HTTPS Server for LAN access
    let https_addr = SocketAddr::from(([0, 0, 0, 0], 443)); // 或者用 0.0.0.0 如果想让所有网卡都监听 HTTPS
    
    let cert_path = "../certs/cert.pem";
    let key_path = "../certs/key.pem";
    
    let rustls_config = RustlsConfig::from_pem_file(cert_path, key_path)
        .await
        .expect("Failed to load SSL certificate");

    info!("HTTPS Server starting on https://{}", https_addr);
    
    axum_server::bind_rustls(https_addr, rustls_config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        // API routes - using new modular structure
        .nest("", api::create_routes())
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
