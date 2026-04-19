use crate::config::Config;
use axum::{
    Router,
    http::{HeaderValue, Method, header},
};
use std::time::Duration;
use tower_http::cors::CorsLayer;

pub fn apply_cors(router: Router, config: &Config) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            config
                .cors
                .allowed_origins
                .iter()
                .map(|o| HeaderValue::from_str(o).unwrap())
                .collect::<Vec<_>>(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true)
        .max_age(Duration::from_secs(config.cors.max_age_secs));

    router.layer(cors)
}
