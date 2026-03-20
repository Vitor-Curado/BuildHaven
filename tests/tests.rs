use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use personal_website::router::app;
use serde_json::Value;
use tower::util::ServiceExt; // for oneshot

const BODY_LIMIT: usize = 64 * 1024; // 64 KB

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let app = app();

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

#[tokio::test]
async fn home_page_renders() {
    let app = app();

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), BODY_LIMIT).await.unwrap();
    let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert!(body_text.contains("Buildhaven"));
}

#[tokio::test]
async fn food_page_renders() {
    let app = app();

    let response = app
        .oneshot(Request::builder().uri("/food").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), BODY_LIMIT).await.unwrap();
    let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();
    // check for some known food title from mock_food_data
    assert!(body_text.contains("Soft-boiled Eggs"));
    assert!(body_text.contains("Scrambled Eggs"));
}

#[tokio::test]
async fn food_detail_existing_and_missing() {
    let app = app();

    // Existing food
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/food/soft-boiled-eggs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = to_bytes(response.into_body(), BODY_LIMIT).await.unwrap();
    let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert!(body_text.contains("Soft-boiled Eggs"));

    // Missing food
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/food/non-existent-food")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn generic_pages_render() {
    let app = app();

    let pages = ["/resume", "/blog", "/contact", "/assets"];
    let expected_titles = ["Resume", "Blog", "Contact", "Assets"];

    for (uri, title) in pages.iter().zip(expected_titles.iter()) {
        let response = app
            .clone()
            .oneshot(Request::builder().uri(*uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), BODY_LIMIT).await.unwrap();
        let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert!(body_text.contains(title));
    }
}
