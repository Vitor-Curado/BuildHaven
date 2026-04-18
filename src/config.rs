#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub environment: Environment,
    pub database_url: String,
    pub allowed_origins: Vec<String>,
    pub db_min_connections: u32,
    pub db_max_connections: u32,
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
        let environment = match std::env::var("APP_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .as_str()
        {
            "production" => Environment::Production,
            "benchmark" => Environment::Benchmark,
            _ => Environment::Development,
        };

        let db_min_connections = std::env::var("DB_MIN_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);

        let db_max_connections = std::env::var("DB_MAX_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(|| Self::default_max_connections(&environment));

        let allowed_origins = std::env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://0.0.0.0:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");


        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self {
            port,
            environment,
            database_url,
            allowed_origins,
            db_min_connections,
            db_max_connections
        }
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
