use crate::config::{Config, Environment};
use axum::Router;
use tower_http::{
    limit::RequestBodyLimitLayer,
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub fn apply_logging(router: Router, config: &Config) -> Router {
    let level = match config.app.environment {
        Environment::Development => Level::DEBUG,
        Environment::Production => Level::INFO,
        Environment::Benchmark => Level::ERROR,
    };
    router
        .layer(RequestBodyLimitLayer::new(config.app.max_request_body_size))
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(level))
                .on_response(DefaultOnResponse::new().level(level))
                .make_span_with(tower_http::trace::DefaultMakeSpan::new().level(level)),
        )
}
