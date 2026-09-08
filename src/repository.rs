use crate::models::{NewPost, Post, User};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_post(pool: &PgPool, new_post: &NewPost) -> Result<Post, sqlx::Error> {
    let id = Uuid::now_v7();

    let post = sqlx::query_as!(
        Post,
        r#"
        INSERT INTO posts (id, title, content)
        VALUES ($1, $2, $3)
        RETURNING id, title, content, created_at, updated_at
        "#,
        id,
        new_post.title,
        new_post.content
    )
    .fetch_one(pool)
    .await?;

    Ok(post)
}

pub async fn get_all_posts(pool: &PgPool) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        SELECT id, title, content, created_at, updated_at
        FROM posts
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_posts_paginated(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        SELECT id, title, content, created_at, updated_at
        FROM posts
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}

pub async fn get_post_by_id(pool: &PgPool, post_id: Uuid) -> Result<Option<Post>, sqlx::Error> {
    let post = sqlx::query_as!(
        Post,
        r#"
        SELECT id, title, content, created_at, updated_at
        FROM posts
        WHERE id = $1
        "#,
        post_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(post)
}

pub async fn delete_post(pool: &PgPool, post_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM posts
        WHERE id = $1
        "#,
        post_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, created_at, updated_at
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
