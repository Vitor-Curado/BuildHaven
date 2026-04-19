use crate::{
    config::Config,
    pool::create_pool,
    router::app,
    state::AppState,
};

use tokio::net::TcpListener;

pub async fn build_listener_and_app(
) -> Result<(TcpListener, axum::Router), Box<dyn std::error::Error>> {
    let config = Config::from_env();

    let db_pool = create_pool(&config).await?;

    let state = AppState::new(db_pool, config)
        .expect("Failed to initialize AppState");

    let port = state.ctx.config.app.port;

    let listener =
        TcpListener::bind(("0.0.0.0", port)).await?;

    let app = app(state);

    Ok((listener, app))
}