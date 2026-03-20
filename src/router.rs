use crate::routes::public_routes;
use crate::state::AppState;
use axum::Router;
use axum::http::header::{HeaderName, HeaderValue};
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

pub fn app() -> Router {
    let static_service = ServeDir::new("static").precompressed_gzip();
    let public_routes = public_routes();

    Router::new()
        .nest("/", public_routes)
        .nest_service("/static", static_service)
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("cache-control"),
            HeaderValue::from_static("public, max-age=86400"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("no-referrer"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("content-security-policy"),
            HeaderValue::from_static("default-src 'self'; img-src 'self' data:; style-src 'self' 'unsafe-inline'; script-src 'self'; font-src 'self';"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("strict-transport-security"),
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("permissions-policy"),
            HeaderValue::from_static(
                "camera=(), microphone=(), geolocation=(), payment=(), usb=(), accelerometer=(), gyroscope=(), magnetometer=()"
            ),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("cross-origin-resource-policy"),
            HeaderValue::from_static("same-origin"),
        ))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .with_state(AppState::new())
}

/*
use axum::Router;
use tower_http::services::ServeDir;

use crate::state::AppState;
use crate::routes::public_routes;
use crate::middleware::apply_middleware;
use crate::security::apply_security_headers;

pub fn app() -> Router {
    let static_service = ServeDir::new("static").precompressed_gzip();

    let router = Router::new()
        .nest("/", public_routes())
        .nest_service("/static", static_service)
        .with_state(AppState::new());

    let router = apply_security_headers(router);
    let router = apply_middleware(router);

    router
}
*/
