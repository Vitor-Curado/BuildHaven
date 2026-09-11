use std::sync::Arc;

use axum::Router;
use buildhaven::{assets::Assets, config::Config, pool::create_pool, router::app, state::AppState};
use tower_sessions_sqlx_store::PostgresStore;

pub async fn test_app() -> Router {
    dotenvy::dotenv().ok();

    let test_database_url =
        std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

    let mut config = Config::from_env().expect("Failed to load test configuration");

    config.database.database_url = test_database_url;

    let db = create_pool(&config)
        .await
        .expect("Failed to create test database pool");

    let session_store = PostgresStore::new(db.clone());

    session_store
        .migrate()
        .await
        .expect("Failed to migrate session store");

    let assets = Assets::new().expect("Failed to load test assets");

    let state = AppState::new(db, Arc::new(config), Arc::new(assets));

    app(state, session_store)
}
