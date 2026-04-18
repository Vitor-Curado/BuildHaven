use crate::{
    config::Environment,
    cors::apply_cors, logging::apply_logging, rate_limit::apply_rate_limiting,
    routes::public_routes, security::apply_security_headers, state::AppState,
};
use axum::Router;
use tower_http::{compression::CompressionLayer, services::ServeDir};

pub fn app(state: AppState) -> Router {
    let static_service = ServeDir::new("static")
        .precompressed_br()
        .precompressed_gzip();

    let router = Router::new()
        .nest("/", public_routes())
        .nest_service("/static", static_service)
        .layer(CompressionLayer::new().br(true).gzip(true).deflate(true))
        .with_state(state.clone());

    let router = apply_security_headers(router);
    let router = match state.ctx.config.environment {
        Environment::Benchmark => router,
        _ => apply_logging(router, &state.ctx.config)
    };

    let router = apply_cors(router, &state.ctx.config);
    apply_rate_limiting(router, &state.ctx.config)
}
