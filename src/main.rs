use buildhaven::{
    config::Config, pool::create_pool, router::app, shutdown::graceful_shutdown_signal,
    state::AppState, telemetry::init_tracing,
};
use dotenvy::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    init_tracing();

    let config = Config::from_env();
    println!("Resolved environment: {:?}", &config.app.environment);
    let db_pool = create_pool(&config).await?;
    let state = AppState::new(db_pool, config).expect("Failed to initialize AppState");

    let port = state.ctx.config.app.port;

    let app = app(state);
    println!("APP_ENV raw: {:?}", std::env::var("APP_ENV"));

    let listener = TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("Failed to bind TCP listener");

    let addr = listener.local_addr().expect("Failed to get local address");

    tracing::info!(
        port = %port,
        %addr,
        "Server listening"
    );

    if let Err(err) = axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown_signal())
        .await
    {
        tracing::error!("Server error: {}", err);
    }

    tracing::info!("Server stopped");

    Ok(())
}
