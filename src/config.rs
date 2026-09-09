use crate::error::{AppError, AppResult};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub max_request_body_size: usize,
    pub cookie_domain: String,
    pub cookie_secure: bool,
}

impl AppConfig {
    pub fn from_env() -> AppResult<Self> {
        envy::from_env().map_err(AppError::Config)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub idle_timeout_secs: u64,
    pub acquire_timeout_secs: u64,
    pub max_lifetime_secs: u64,
}

impl DatabaseConfig {
    pub fn from_env() -> AppResult<Self> {
        envy::from_env().map_err(AppError::Config)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: String,
    pub max_age_secs: u64,
}

impl CorsConfig {
    pub fn from_env() -> AppResult<Self> {
        envy::from_env().map_err(AppError::Config)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SecurityConfig {
    pub cache_control: String,
    pub content_security_policy: String,
    pub hsts_enabled: bool,
}

impl SecurityConfig {
    pub fn from_env() -> AppResult<Self> {
        envy::from_env().map_err(AppError::Config)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SessionConfig {
    pub duration_hours: i64
}

impl SessionConfig {
    pub fn from_env() -> AppResult<Self> {
        envy::from_env().map_err(AppError::Config)
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub cors: CorsConfig,
    pub session: SessionConfig,
    pub security: SecurityConfig,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        Ok(Self {
            app: AppConfig::from_env()?,
            database: DatabaseConfig::from_env()?,
            cors: CorsConfig::from_env()?,
            session: SessionConfig::from_env()?,
            security: SecurityConfig::from_env()?,
        })
    }
}
