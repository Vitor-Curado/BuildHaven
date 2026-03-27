use crate::cors::apply_cors;
use crate::logging::apply_logging;
use crate::rate_limit::apply_rate_limiting;
use crate::routes::public_routes;
use crate::security::apply_security_headers;
use crate::state::AppState;

use axum::Router;
use tower_http::services::ServeDir;

pub fn app(state: AppState) -> Router {
    let static_service = ServeDir::new("static").precompressed_gzip();

    let router = Router::new()
        .nest("/", public_routes())
        .nest_service("/static", static_service)
        .with_state(state);

    let router = apply_security_headers(router);
    let router = apply_logging(router);
    let router = apply_cors(router);
    apply_rate_limiting(router)
}
