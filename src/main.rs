use personal_website::router::app;
use tokio::net::TcpListener;
use personal_website::config::Config;

#[tokio::main]
async fn main() {
    let app = app();
    let config = Config::from_env();
    let listener = TcpListener::bind(("0.0.0.0", config.port)).await.unwrap();
    // let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
