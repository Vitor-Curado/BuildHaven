use axum::Router;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

pub fn apply_middleware(router: Router) -> Router {
    router
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
}