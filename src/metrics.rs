use axum::{
    extract::{MatchedPath, Request},
    http::header,
    middleware::Next,
    response::{IntoResponse, Response},
};

use metrics::{counter, describe_counter, describe_gauge, describe_histogram, gauge, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::time::Instant;
use tokio::time::{self, Duration};

pub fn init_metrics() -> PrometheusHandle {
    describe_counter!("http_requests_total", "Total number of HTTP requests.");

    describe_histogram!(
        "http_request_duration_seconds",
        "HTTP request duration in seconds."
    );

    describe_gauge!(
        "http_requests_in_flight",
        "Number of HTTP requests currently being processed."
    );

    let handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install Prometheus metrics recorder");

    let upkeep_handle = handle.clone();

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));

        loop {
            interval.tick().await;
            upkeep_handle.run_upkeep();
        }
    });

    handle
}

pub async fn metrics_endpoint(
    axum::extract::Extension(handle): axum::extract::Extension<PrometheusHandle>,
) -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4")],
        handle.render(),
    )
}

pub async fn record_http_metrics(request: Request, next: Next) -> Response {
    if request.uri().path() == "/metrics" {
        return next.run(request).await;
    }

    let method = request.method().clone();
    let route = request
        .extensions()
        .get::<MatchedPath>()
        .map(|path| path.as_str().to_owned())
        .unwrap_or_else(|| "unmatched".to_owned());
    let in_flight = gauge!(
        "http_requests_in_flight",
        "method" => method.to_string(),
        "route" => route.clone()
    );

    in_flight.increment(1.0);

    let start = Instant::now();
    let response = next.run(request).await;
    let duration = start.elapsed().as_secs_f64();

    in_flight.decrement(1.0);

    let status = response.status().as_u16().to_string();
    let method = method.as_str();

    counter!(
        "http_requests_total",
        "method" => method.to_owned(),
        "route" => route.clone(),
        "status" => status.clone()
    )
    .increment(1);

    histogram!(
        "http_request_duration_seconds",
        "method" => method.to_owned(),
        "route" => route,
        "status" => status
    )
    .record(duration);

    response
}
