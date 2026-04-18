use crate::config::Config;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn create_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .min_connections(config.db_min_connections)
        .idle_timeout(Duration::from_secs(300))
        .acquire_timeout(Duration::from_secs(5))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&config.database_url)
        .await
}