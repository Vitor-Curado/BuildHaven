use crate::{
    auth::require_auth,
    handlers::{
        admin, admin_create_post, admin_delete_post, admin_edit_post, admin_new_post, admin_posts,
        admin_update_post, blog, blog_post, contact, docs, home, login_page, login_user, resume,
    },
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route("/docs/{slug}", get(docs))
        .route("/login", get(login_page).post(login_user))
        .route("/resume", get(resume))
        .route("/blog", get(blog))
        .route("/blog/{slug}", get(blog_post))
        .route("/contact", get(contact))
}

// Todo: Add authenticated routes here in the future
pub fn protected_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/admin", get(admin))
        .route("/admin/posts", get(admin_posts).post(admin_create_post))
        .route("/admin/posts/new", get(admin_new_post))
        .route(
            "/admin/posts/{id}",
            get(admin_edit_post).post(admin_update_post),
        )
        .route("/admin/posts/{id}/delete", post(admin_delete_post))
        .layer(axum::middleware::from_fn_with_state(state, require_auth))
}
