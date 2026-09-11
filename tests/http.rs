// HTTP ↔ router ↔ handlers ↔ repository
mod common;

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use common::test_app;
use tower::util::ServiceExt;

const BODY_LIMIT: usize = 64 * 1024;

#[tokio::test]
async fn home_page_renders() {
    let app = test_app().await;

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), BODY_LIMIT).await.unwrap();

    let body = String::from_utf8(body.to_vec()).unwrap();

    assert!(body.contains("Buildhaven"));
}
