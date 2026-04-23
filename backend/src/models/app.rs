use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Application model representing an isolated SES deployment
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct App {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub database_url: String,
    pub config: Option<serde_json::Value>,
    pub status: AppStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "app_status", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum AppStatus {
    Active,
    Inactive,
    Suspended,
}

/// Request to create a new app
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAppRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Request to update an app
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAppRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<AppStatus>,
}

/// App response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: AppStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<App> for AppResponse {
    fn from(app: App) -> Self {
        Self {
            id: app.id,
            name: app.name,
            description: app.description,
            status: app.status,
            created_at: app.created_at,
            updated_at: app.updated_at,
        }
    }
}
