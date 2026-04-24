use std::sync::Arc;

use crate::{
    assets::Assets, config::Config, error::AppResult, jobs::JobRunner, metrics::init_build_info,
    pool::create_pool, router::app, state::AppState,
};

use tokio::net::TcpListener;

pub async fn build_listener_and_app() -> AppResult<(TcpListener, axum::Router)> {
    let config = Arc::new(Config::from_env()?);
    let db_pool = create_pool(&config).await?;
    let assets = Assets::build()?;
    let state = AppState::new(db_pool, config, assets)?;

    init_build_info();

    let job_runner = JobRunner::new(state.clone());
    job_runner.start();

    let port = state.ctx.config.app.port;

    let listener = TcpListener::bind(("0.0.0.0", port)).await?;

    let app = app(state);

    Ok((listener, app))
}
