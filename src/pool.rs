use crate::config::Config;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn create_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .idle_timeout(Duration::from_secs(config.database.idle_timeout_secs))
        .acquire_timeout(Duration::from_secs(config.database.acquire_timeout_secs))
        .max_lifetime(Duration::from_secs(config.database.max_lifetime_secs))
        .connect(&config.database.url)
        .await
}