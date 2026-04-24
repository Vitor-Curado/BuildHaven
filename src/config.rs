use crate::error::{AppError, AppResult};

#[derive(Clone)]
pub struct AppConfig {
    pub port: u16,
    pub environment: Environment,
    pub max_request_body_size: usize,
    pub cookie_domain: String,
}

impl AppConfig {
    pub fn from_env() -> AppResult<Self> {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| AppError::Internal)?;

        let environment = Environment::from_env();

        let max_request_body_size = std::env::var("MAX_REQUEST_BODY_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(|| Self::default_request_body_size(&environment));

        let cookie_domain =
            std::env::var("COOKIE_DOMAIN").unwrap_or_else(|_| "localhost".to_string());

        Ok(Self {
            port,
            environment,
            max_request_body_size,
            cookie_domain,
        })
    }

    fn default_request_body_size(env: &Environment) -> usize {
        match env {
            Environment::Development => 10 * 1024 * 1024, // 10 MB
            Environment::Benchmark => 50 * 1024 * 1024,   // 50 MB
            Environment::Production => 2 * 1024 * 1024,   // 2 MB
        }
    }
}

#[derive(Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub idle_timeout_secs: u64,
    pub acquire_timeout_secs: u64,
    pub max_lifetime_secs: u64,
}

impl DatabaseConfig {
    pub fn from_env(env: &Environment) -> AppResult<Self> {
        let url = std::env::var("DATABASE_URL").map_err(|_| AppError::Internal)?;

        let min_connections = std::env::var("DB_MIN_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);

        let max_connections = std::env::var("DB_MAX_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(|| Self::default_max_connections(env));

        let idle_timeout_secs = std::env::var("DB_IDLE_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(300);

        let acquire_timeout_secs = std::env::var("DB_ACQUIRE_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

        let max_lifetime_secs = std::env::var("DB_MAX_LIFETIME_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1800);

        Ok(Self {
            url,
            min_connections,
            max_connections,
            idle_timeout_secs,
            acquire_timeout_secs,
            max_lifetime_secs,
        })
    }

    fn default_max_connections(env: &Environment) -> u32 {
        match env {
            Environment::Development => 5,
            Environment::Benchmark => 20,
            Environment::Production => {
                let cores = num_cpus::get();
                let calc = (cores * 2) as u32;
                calc.clamp(2, 10)
            }
        }
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
pub struct RateLimitConfig {
    pub per_second: u64,
    pub burst_size: u32,
}

impl RateLimitConfig {
    pub fn from_env() -> Self {
        let per_second = std::env::var("RATE_LIMIT_PER_SECOND")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);

        let burst_size = std::env::var("RATE_LIMIT_BURST_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);

        Self {
            per_second,
            burst_size,
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
    pub rate_limit: RateLimitConfig,
    pub session: SessionConfig,
    pub security: SecurityConfig,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        let app = AppConfig::from_env()?;
        Ok(Self {
            database: DatabaseConfig::from_env(&app.environment)?,
            cors: CorsConfig::from_env(),
            rate_limit: RateLimitConfig::from_env(),
            session: SessionConfig::from_env(),
            security: SecurityConfig::from_env(),
            app,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Environment {
    Development,
    Production,
    Benchmark,
}

impl Environment {
    fn from_env() -> Self {
        match std::env::var("APP_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str()
        {
            "production" => Self::Production,
            "benchmark" => Self::Benchmark,
            _ => Self::Development,
        }
    }
}
