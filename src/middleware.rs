use std::time::Instant;
use uuid::Uuid;

use axum::{extract::Request, middleware::Next, response::Response};

pub async fn latency_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let start = Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed();

    tracing::info!(
        method = %method,
        uri = %uri,
        latency_ms = duration.as_millis(),
        status = %response.status(),
        "request completed"
    );

    response
}

pub async fn request_id_middleware(mut req: Request, next: Next) -> Response {
    let request_id = Uuid::new_v4();
    let method = req.method().clone();
    let uri = req.uri().clone();

    req.extensions_mut().insert(request_id);

    tracing::info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        "request started"
    );

    let response = next.run(req).await;

    tracing::info!(
        request_id = %request_id,
        "request finished"
    );

    response
}