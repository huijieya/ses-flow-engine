use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SesError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Flow execution error: {0}")]
    FlowExecution(String),

    #[error("Node execution error: {0}")]
    NodeExecution(String),

    #[error("Device error: {0}")]
    Device(String),

    #[error("Event bus error: {0}")]
    EventBus(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
}

impl IntoResponse for SesError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            SesError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            SesError::Redis(e) => {
                tracing::error!("Redis error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Cache error".to_string())
            }
            SesError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            SesError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            SesError::FlowExecution(msg) => {
                tracing::error!("Flow execution error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            SesError::NodeExecution(msg) => {
                tracing::error!("Node execution error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            SesError::Device(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            SesError::EventBus(msg) => {
                tracing::error!("Event bus error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            SesError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            SesError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            SesError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            SesError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            SesError::Config(e) => {
                tracing::error!("Configuration error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error".to_string())
            }
        };

        let body = Json(json!({
            "success": false,
            "error": {
                "code": status.as_u16(),
                "message": error_message,
            }
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, SesError>;
