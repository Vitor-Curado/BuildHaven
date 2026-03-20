use crate::handlers::{assets, blog, contact, food, food_detail, health, home, resume};
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/food", get(food))
        .route("/food/:slug", get(food_detail))
        .route("/resume", get(resume))
        .route("/blog", get(blog))
        .route("/contact", get(contact))
        .route("/assets", get(assets))
}
