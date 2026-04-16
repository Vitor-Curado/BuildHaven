use crate::{
    handlers::{assets, blog, contact, food, food_detail, health, home, resume, register_page, register_user, login_page, login_user},
    state::AppState,
};
use axum::{Router, routing::{get, post}};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route("/health", get(health))

        // Register
        .route("/register",
            get(register_page)
            .post(register_user)
        )

        // Login
        .route("/login",
            get(login_page)
            .post(login_user)
        )

        .route("/resume", get(resume))
        .route("/blog", get(blog))
        .route("/contact", get(contact))
        .route("/food", get(food))
        .route("/food/:slug", get(food_detail))
        .route("/assets", get(assets))
}
