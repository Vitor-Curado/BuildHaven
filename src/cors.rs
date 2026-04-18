use crate::config::Config;
use axum::{
    Router,
    http::{HeaderValue, Method, header},
};
use std::time::Duration;
use tower_http::cors::CorsLayer;

pub fn apply_cors(router: Router, config: &Config) -> Router {
    let origin = HeaderValue::from_str(&config.allowed_origins[0]).expect("Invalid ALLOWED_ORIGIN");

    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    router.layer(cors)
}
