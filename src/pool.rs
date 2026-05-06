use crate::config::Config;
use sqlx::{Connection, PgPool, postgres::PgPoolOptions};
use tokio::time::Duration;

async fn wait_for_db(url: &str) {
    for attempt in 1..=40 {
        match sqlx::postgres::PgConnection::connect(url).await {
            Ok(mut conn) => {
                let _ = sqlx::query("SELECT 1").execute(&mut conn).await;
                return;
            }
            Err(e) => {
                eprintln!("DB not ready ({attempt}/40): {e:#}");
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }

    panic!("Database never became ready");
}

pub async fn create_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    wait_for_db(&config.database.url).await;

    PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .idle_timeout(Duration::from_secs(config.database.idle_timeout_secs))
        .acquire_timeout(Duration::from_secs(config.database.acquire_timeout_secs))
        .max_lifetime(Duration::from_secs(config.database.max_lifetime_secs))
        .connect(&config.database.url)
        .await
}
