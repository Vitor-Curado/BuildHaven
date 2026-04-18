#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub environment: Environment,
    pub database_url: String,
    pub allowed_origins: Vec<String>,
}

#[derive(Clone)]
pub enum Environment {
    Development,
    Production,
    Benchmark,
}

impl Config {
    #[must_use]
    pub fn from_env() -> Self {
        let allowed_origins = std::env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://0.0.0.0:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");

        let environment = match std::env::var("APP_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .as_str()
        {
            "production" => Environment::Production,
            "benchmark" => Environment::Benchmark,
            _ => Environment::Development,
        };

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self {
            port,
            environment,
            database_url,
            allowed_origins,
        }
    }
}
