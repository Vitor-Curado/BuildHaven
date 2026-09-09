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
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
use time::Duration;

pub fn app(state: AppState, session_store: PostgresStore) -> Router {
    let config = &state.config;
    let static_service = ServeDir::new("static")
        .precompressed_br()
        .precompressed_gzip();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_name(crate::constants::cookies::SESSION_ID)
        .with_http_only(true)
        .with_secure(config.app.cookie_secure)
        .with_same_site(tower_sessions::cookie::SameSite::Strict)
        .with_path("/")
        .with_expiry(Expiry::OnInactivity(Duration::hours(config.session.duration_hours)));

    let mut router = Router::new()
        .merge(public_routes())
        .layer(session_layer)
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
