use crate::routes::public_routes;
use crate::state::AppState;
use crate::middleware::apply_middleware;
use crate::security::apply_security_headers;

use axum::Router;
use tower_http::services::ServeDir;

pub fn app(state: AppState) -> Router {
    let static_service =
        ServeDir::new("static")
            .precompressed_gzip();

    let router = Router::new()
        .nest("/", public_routes())
        .nest_service("/static", static_service)
        .with_state(state);

    let router = apply_security_headers(router);

    let router = apply_middleware(router);

    router
}