use axum::Router;
use tower_http::{limit::RequestBodyLimitLayer, trace::TraceLayer};
use tracing::Level;

pub fn apply_logging(router: Router) -> Router {
    router.layer(RequestBodyLimitLayer::new(1024 * 1024)).layer(
        TraceLayer::new_for_http()
            .make_span_with(tower_http::trace::DefaultMakeSpan::new().level(Level::INFO)),
    )
}
