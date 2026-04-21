use crate::{
    config::Environment,
    cors::apply_cors,
    logging::apply_logging,
    middleware::{latency_middleware, request_id_middleware},
    rate_limit::apply_rate_limiting,
    routes::public_routes,
    security::apply_security_headers,
    state::AppState,
};
use axum::Router;
use tower_http::{compression::CompressionLayer, services::ServeDir};

pub fn app(state: AppState) -> Router {
    let config = &state.ctx.config;
    let static_service = ServeDir::new("static")
        .precompressed_br()
        .precompressed_gzip();

    let mut router = Router::new()
        .merge(public_routes())
        .nest_service("/static", static_service)
        .layer(CompressionLayer::new().br(true).gzip(true).deflate(true))
        .with_state(state.clone());

    if !matches!(config.app.environment, Environment::Benchmark) {
        router = router
            .layer(axum::middleware::from_fn(request_id_middleware))
            .layer(axum::middleware::from_fn(latency_middleware))
    }

    router = apply_security_headers(router, config);

    if !matches!(config.app.environment, Environment::Benchmark) {
        router = apply_logging(router, config);
        router = apply_rate_limiting(router, config);
    }

    router = apply_cors(router, config);
    router
}
