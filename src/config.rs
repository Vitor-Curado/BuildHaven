pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");

        Self { port }
    }
}