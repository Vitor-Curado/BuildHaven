use axum::Router;
use buildhaven::{
    config::{Config, Environment},
    pool::create_pool,
    router::app,
    state::AppState,
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

/// Creates a fresh database pool for tests.
/// Expects TEST_DATABASE_URL in environment.
pub async fn setup_test_db() -> PgPool {
    dotenvy::dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    sqlx::migrate!()
    .run(&pool)
    .await
    .expect("Failed to run migrations");

    // Clean posts table before each test
    sqlx::query("TRUNCATE posts RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await
        .expect("Failed to truncate posts table");

    pool
}

pub async fn test_app() -> Router {
    dotenvy::dotenv().ok();

    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

    let db = create_pool(&database_url).await;

    let config = Config {
        port: 0,
        environment: Environment::Development,
        database_url,
    };

    let state = AppState::new(db, config).expect("Failed to build test AppState");

    app(state)
}
