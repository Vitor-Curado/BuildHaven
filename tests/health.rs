mod common;
use common::test_app;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use tower::util::ServiceExt;
use serde_json::Value;

const BODY_LIMIT: usize = 64 * 1024; // 64 KB

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), BODY_LIMIT).await.unwrap();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(json["status"], "ok");
    assert_eq!(json["service"], "personal-website");
    assert!(json["version"].as_str().unwrap().is_empty());
}