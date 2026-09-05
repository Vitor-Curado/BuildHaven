use crate::error::{AppError, AppResult};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub max_request_body_size: usize,
    pub cookie_domain: String,
    pub cookie_secure: bool,
}

impl AppConfig {
    pub fn from_env() -> AppResult<Self> {
        envy::from_env().map_err(|_| AppError::Internal)
    }
}

#[derive(Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub idle_timeout_secs: u64,
    pub acquire_timeout_secs: u64,
    pub max_lifetime_secs: u64,
}

impl DatabaseConfig {
    pub fn from_env() -> AppResult<Self> {
        let url = std::env::var("DATABASE_URL").map_err(|_| AppError::Internal)?;

        Ok(Self {
            url,
            min_connections: Self::default_db_min_connections(),
            max_connections: Self::default_db_max_connections(),
            idle_timeout_secs: Self::default_db_idle_timeout_secs(),
            acquire_timeout_secs: Self::default_db_acquire_timeout_secs(),
            max_lifetime_secs: Self::default_db_max_lifetime_secs(),
        })
    }

    fn default_db_min_connections() -> u32 {
        1
    }

    fn default_db_max_connections() -> u32 {
        10
    }

    fn default_db_idle_timeout_secs() -> u64 {
        300
    }

    fn default_db_acquire_timeout_secs() -> u64 {
        30
    }

    fn default_db_max_lifetime_secs() -> u64 {
        1800
    }
}

#[derive(Clone)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub max_age_secs: u64,
}

impl CorsConfig {
    pub fn from_env() -> Self {
        let allowed_origins = std::env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://0.0.0.0:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let max_age_secs = std::env::var("CORS_MAX_AGE_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(86400); // 24 hours

        Self {
            allowed_origins,
            max_age_secs,
        }
    }
}

#[derive(Clone)]
pub struct SecurityConfig {
    pub cache_control: String,
    pub content_security_policy: String,
    pub hsts_enabled: bool,
}

impl SecurityConfig {
    pub fn from_env() -> Self {
        let cache_control = std::env::var("CACHE_CONTROL")
            .unwrap_or_else(|_| "public, max-age=31536000, immutable".to_string());

        let content_security_policy = std::env::var("CONTENT_SECURITY_POLICY")
            .unwrap_or_else(|_| "default-src 'self';".to_string());

        let hsts_enabled = std::env::var("HSTS_ENABLED")
            .map(|v| v == "true")
            .unwrap_or(false);

        Self {
            cache_control,
            content_security_policy,
            hsts_enabled,
        }
    }
}

#[derive(Clone)]
pub struct SessionConfig {
    pub duration_hours: i64,
}

impl SessionConfig {
    pub fn from_env() -> Self {
        let duration_hours = std::env::var("SESSION_DURATION_HOURS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(24);

        Self { duration_hours }
    }
}

#[derive(Clone)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub cors: CorsConfig,
    pub session: SessionConfig,
    pub security: SecurityConfig,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        let app = AppConfig::from_env()?;
        Ok(Self {
            database: DatabaseConfig::from_env()?,
            cors: CorsConfig::from_env(),
            session: SessionConfig::from_env(),
            security: SecurityConfig::from_env(),
            app,
        })
    }
}
