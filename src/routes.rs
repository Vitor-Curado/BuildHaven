use crate::{
    auth::require_auth,
    constants,
    handlers::{
        admin, admin_create_post, admin_delete_post, admin_edit_post, admin_new_post, admin_posts,
        admin_publish_post, admin_unpublish_post, admin_update_post, blog_post, home, login_page,
        login_user,
    },
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route(constants::endpoints::HOME, get(home))
        .route(constants::endpoints::BLOG_POST, get(blog_post))
        .route(
            constants::endpoints::LOGIN,
            get(login_page).post(login_user),
        )
}

pub fn protected_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(constants::endpoints::ADMIN, get(admin))
        .route(
            constants::endpoints::ADMIN_POSTS,
            get(admin_posts).post(admin_create_post),
        )
        .route(constants::endpoints::ADMIN_NEW_POST, get(admin_new_post))
        .route(
            constants::endpoints::ADMIN_POST,
            get(admin_edit_post).post(admin_update_post),
        )
        .route(
            constants::endpoints::ADMIN_DELETE_POST,
            post(admin_delete_post),
        )
        .route(
            constants::endpoints::ADMIN_PUBLISH_POST,
            post(admin_publish_post),
        )
        .route(
            constants::endpoints::ADMIN_UNPUBLISH_POST,
            post(admin_unpublish_post),
        )
        .layer(axum::middleware::from_fn_with_state(state, require_auth))
}
