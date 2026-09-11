use crate::{
    handlers::{blog, contact, docs, home, login_page, login_user, resume},
    state::AppState,
};
use axum::{Router, routing::get};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route("/docs/{slug}", get(docs))
        .route("/login", get(login_page).post(login_user))
        .route("/resume", get(resume))
        .route("/blog", get(blog))
        .route("/contact", get(contact))
}

// Todo: Add authenticated routes here in the future
pub fn protected_routes() -> Router<AppState> {
    Router::new()
}
