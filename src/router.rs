use crate::{
    cors::apply_cors, logging::apply_logging, routes::public_routes,
    security::apply_security_headers, state::AppState,
};
use axum::Router;
use tower_http::{
    compression::CompressionLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    services::ServeDir,
};

pub fn app(state: AppState) -> Router {
    let config = &state.config;
    let static_service = ServeDir::new("static")
        .precompressed_br()
        .precompressed_gzip();

    let mut router = Router::new()
        .merge(public_routes())
        .nest_service("/static", static_service)
        .layer(CompressionLayer::new().br(true).gzip(true).deflate(true))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        .with_state(state.clone());

    router = apply_security_headers(router, config);
    router = apply_logging(router, config);
    router = apply_cors(router, config);

    router
}
