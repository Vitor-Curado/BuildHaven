use buildhaven::{
    bootstrap::build_listener_and_app, error::AppError, metrics::init_start_time,
    shutdown::graceful_shutdown_signal, telemetry::init_tracing,
};

use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    init_start_time();
    dotenv().ok();
    init_tracing();

    let (listener, app) = build_listener_and_app().await?;

    tracing::info!(
        addr = %listener.local_addr()?,
        "Server listening"
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown_signal())
        .await?;

    tracing::info!("Server stopped");

    Ok(())
}
