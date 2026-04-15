pub struct Config {
    pub port: u16,
    pub environment: Environment,
    pub database_url: String,
}

pub enum Environment {
    Development,
    Production,
}

impl Config {
    #[must_use]
    pub fn from_env() -> Self {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");

        let environment = match std::env::var("APP_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .as_str()
        {
            "production" => Environment::Production,
            _ => Environment::Development,
        };

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self {
            port,
            environment,
            database_url,
        }
    }
}
