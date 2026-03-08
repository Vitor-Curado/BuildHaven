use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;

use crate::handlers::{assets, blog, contact, food, food_detail, health, home, resume};

pub fn app() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/food", get(food))
        .route("/food/:slug", get(food_detail))
        .route("/resume", get(resume))
        .route("/blog", get(blog))
        .route("/contact", get(contact))
        .route("/assets", get(assets))
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/media", ServeDir::new("media"))
}
