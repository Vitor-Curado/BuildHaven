use crate::{
    handlers::{
        assets, blog, contact, food, food_detail, health, home, login_page, login_user,
        register_page, register_user, resume,
    },
    metrics::gather_metrics,
    state::AppState,
};
use axum::{Router, response::IntoResponse, routing::get};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/metrics", get(metrics))
        // Register
        .route("/register", get(register_page).post(register_user))
        // Login
        .route("/login", get(login_page).post(login_user))
        .route("/resume", get(resume))
        .route("/blog", get(blog))
        .route("/contact", get(contact))
        .route("/food", get(food))
        .route("/food/:slug", get(food_detail))
        .route("/assets", get(assets))
}

// Todo: Add authenticated routes here in the future
pub fn protected_routes() -> Router<AppState> {
    Router::new()
}

async fn metrics() -> impl IntoResponse {
    (
        [("Content-Type", "text/plain; version=0.0.4")],
        gather_metrics().into_response(),
    )
}
