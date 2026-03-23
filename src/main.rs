use personal_website::config::Config;
use personal_website::pool::create_pool;
use personal_website::router::app;
use personal_website::state::AppState;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = create_pool(&database_url).await;

    let state = AppState::new(db_pool);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = app(state);
    let config = Config::from_env();
    let listener = TcpListener::bind(("0.0.0.0", config.port)).await.unwrap();
    // let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
