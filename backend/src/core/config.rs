use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://gegewu:qwe123@localhost:5432/ses_db".to_string(),
            max_connections: 10,
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
        }
    }
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "your-secret-key-change-in-production".to_string(),
            expiration_hours: 24,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            // Start with defaults - use string keys instead of structs
            .set_default("server.host", ServerConfig::default().host)?
            .set_default("server.port", ServerConfig::default().port as i64)?
            .set_default("database.url", DatabaseConfig::default().url)?
            .set_default("database.max_connections", DatabaseConfig::default().max_connections as i64)?
            .set_default("redis.url", RedisConfig::default().url)?
            .set_default("jwt.secret", JwtConfig::default().secret)?
            .set_default("jwt.expiration_hours", JwtConfig::default().expiration_hours)?
            // Add config file (optional)
            .add_source(File::with_name("config/config").required(false))
            // Add environment variables with prefix SES_
            .add_source(Environment::with_prefix("SES").separator("__"));

        let config = builder.build()?;
        config.try_deserialize()
    }
}
