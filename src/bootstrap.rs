use std::sync::Arc;
use tower_sessions_sqlx_store::PostgresStore;

use crate::{
    assets::Assets, config::Config, error::AppResult, pool::create_pool, router::app,
    state::AppState,
};

use tokio::net::TcpListener;

pub async fn build_listener_and_app() -> AppResult<(TcpListener, axum::Router)> {
    let config = Arc::new(Config::from_env()?);
    let db_pool = create_pool(&config).await?;
    let session_store = PostgresStore::new(db_pool.clone());
    session_store.migrate().await?;
    let assets = Arc::new(Assets::build()?);
    let state = AppState::new(db_pool, config, assets);

    let port = state.config.app.port;

    let listener = TcpListener::bind(("0.0.0.0", port)).await?;

    let app = app(state, session_store);

    Ok((listener, app))
}
