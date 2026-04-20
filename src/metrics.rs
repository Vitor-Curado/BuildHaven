// Consider: switching to VictoriaMetrics

use once_cell::sync::Lazy;
use prometheus::{
    Encoder, HistogramVec, IntCounterVec, IntGaugeVec, Opts, Registry, TextEncoder,
};

pub static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

pub static HTTP_REQUESTS: Lazy<IntCounterVec> = Lazy::new(|| {
    let counter = IntCounterVec::new(
        prometheus::Opts::new("http_requests_total", "Total HTTP requests"),
        &["method", "path", "status"],
    )
    .unwrap();

    REGISTRY.register(Box::new(counter.clone())).unwrap();
    counter
});

pub static HTTP_LATENCY: Lazy<HistogramVec> = Lazy::new(|| {
    let histogram = HistogramVec::new(
        prometheus::HistogramOpts::new("http_request_duration_seconds", "HTTP latency").buckets(
            vec![
                0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0,
            ],
        ),
        &["method", "path"],
    )
    .unwrap();

    REGISTRY.register(Box::new(histogram.clone())).unwrap();
    histogram
});

pub fn gather_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();

    let mut buffer = Vec::new();

    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

pub static HTTP_INFLIGHT: Lazy<IntGaugeVec> = Lazy::new(|| {
    let gauge = IntGaugeVec::new(
        prometheus::Opts::new(
            "http_requests_in_flight",
            "Number of HTTP requests currently in flight",
        ),
        &["method", "path"],
    )
    .unwrap();

    REGISTRY.register(Box::new(gauge.clone())).unwrap();

    gauge
});

pub static HTTP_ERRORS: Lazy<IntCounterVec> = Lazy::new(|| {
    let counter = IntCounterVec::new(
        prometheus::Opts::new("http_errors_total", "Total HTTP error responses"),
        &["method", "path", "status"],
    )
    .unwrap();

    REGISTRY.register(Box::new(counter.clone())).unwrap();

    counter
});

pub static BUILD_INFO: Lazy<IntGaugeVec> = Lazy::new(|| {
    let gauge = IntGaugeVec::new(Opts::new("build_info", "Build information"), &["version"]).unwrap();

    REGISTRY.register(Box::new(gauge.clone())).unwrap();

    gauge
});

pub fn init_build_info() {
    let version =
        env!("CARGO_PKG_VERSION");

    BUILD_INFO
        .with_label_values(&[version])
        .set(1);
}

pub fn normalize_path(path: &str) -> String {
    if path.starts_with("/food/") {
        "/food/:slug".into()
    } else {
        path.into()
    }
}
