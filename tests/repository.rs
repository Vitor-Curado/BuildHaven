mod common;
use common::setup_test_db;
use buildhaven::models::NewPost;
use buildhaven::repository;

#[tokio::test]
async fn create_and_fetch_post() {
    let db = setup_test_db().await;

    let new_post = NewPost {
        title: "Test Post".into(),
        content: "Hello world".into(),
    };

    let created =
        repository::create_post(&db, &new_post)
            .await
            .unwrap();

    let fetched =
        repository::get_post_by_id(&db, created.id)
            .await
            .unwrap()
            .unwrap();

    assert_eq!(fetched.title, "Test Post");
}