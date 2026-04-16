use buildhaven::{config::Config, pool::create_pool, router::app, state::AppState};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::from_env();
    let db_pool = create_pool(&config.database_url).await;
    let state = AppState::new(db_pool, config).expect("Failed to initialize AppState");

    let port = state.config.port;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = app(state);

    let listener = TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("Failed to bind TCP listener");
    // let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!(
        "Server listening on {}",
        listener.local_addr().expect("Failed to get local address")
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen for shutdown signal");
        })
        .await
        .unwrap();
}
