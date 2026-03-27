use axum::Router;
use axum::http::{Method, HeaderValue};
use tower_http::cors::{Any, CorsLayer};

pub fn apply_cors(router: Router) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            HeaderValue::from_static("https://buildhaven.cc")
        )
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    router.layer(cors)
}
