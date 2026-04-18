use buildhaven::{
    config::Config, pool::create_pool, router::app, shutdown::graceful_shutdown_signal,
    state::AppState,
};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    println!("Resolved environment: {:?}", &config.environment);
    let db_pool = create_pool(&config).await?;
    let state = AppState::new(db_pool, config).expect("Failed to initialize AppState");

    let port = state.ctx.config.port;

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
