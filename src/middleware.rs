use crate::metrics::{HTTP_ERRORS, HTTP_INFLIGHT, HTTP_LATENCY, HTTP_REQUESTS, normalize_path};

use axum::{extract::Request, middleware::Next, response::Response};

use std::time::Instant;
use uuid::Uuid;

pub async fn latency_middleware(req: Request, next: Next) -> Response {
    let method = req.method().to_string();
    let path = normalize_path(req.uri().path());

    HTTP_INFLIGHT
        .with_label_values(&[method.as_str(), path.as_str()])
        .inc();

    let start = Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed();

    let status = response.status().as_u16();

    let elapsed = duration.as_secs_f64();

    HTTP_LATENCY
        .with_label_values(&[method.as_str(), path.as_str()])
        .observe(elapsed);

    HTTP_REQUESTS
        .with_label_values(&[method.as_str(), path.as_str(), status.to_string().as_str()])
        .inc();

    if status >= 400 {
        HTTP_ERRORS
            .with_label_values(&[method.as_str(), path.as_str(), status.to_string().as_str()])
            .inc();
    }

    HTTP_INFLIGHT
        .with_label_values(&[method.as_str(), path.as_str()])
        .dec();

    tracing::info!(
        method = %method,
        uri = %path,
        latency_ms = duration.as_millis(),
        status = %status,
        "request completed"
    );

    response
}

pub async fn request_id_middleware(mut req: Request, next: Next) -> Response {
    let request_id = Uuid::now_v7();
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
