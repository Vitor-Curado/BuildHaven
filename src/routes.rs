use crate::{
    handlers::{assets, blog, contact, food, food_detail, health, home, resume},
    state::AppState,
};
use axum::{Router, routing::get};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/resume", get(resume))
        .route("/blog", get(blog))
        .route("/contact", get(contact))
        .route("/food", get(food))
        .route("/food/:slug", get(food_detail))
        .route("/assets", get(assets))
}
