use crate::models::{NewPost, Post, UpdatePost, User};
use slug::slugify;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_post(pool: &PgPool, new_post: &NewPost) -> Result<Post, sqlx::Error> {
    let id = Uuid::now_v7();
    let slug = slugify(&new_post.title);

    let post = sqlx::query_as!(
        Post,
        r#"
        INSERT INTO posts (id, title, slug, content)
        VALUES ($1, $2, $3, $4)
        RETURNING 
            id, 
            title, 
            slug,
            content,
            status,
            views,
            published_at, 
            created_at, 
            updated_at
        "#,
        id,
        new_post.title,
        slug,
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
        SELECT 
            id, 
            title, 
            slug,
            content,
            status,
            views,
            published_at, 
            created_at, 
            updated_at
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
        SELECT 
            id, 
            title, 
            slug, 
            content, 
            status, 
            views, 
            published_at, 
            created_at, 
            updated_at
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
        SELECT 
            id, 
            title, 
            slug,
            content, 
            status,
            views,
            published_at,
            created_at, 
            updated_at
        FROM posts
        WHERE id = $1
        "#,
        post_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(post)
}

pub async fn get_posts_by_slug(pool: &PgPool, slug: String) -> Result<Option<Post>, sqlx::Error> {
    let post = sqlx::query_as!(
        Post,
        r#"
        SELECT 
            id, 
            title, 
            slug,
            content, 
            status,
            views,
            published_at,
            created_at, 
            updated_at
        FROM posts
        where slug = $1
        AND status = 'published'
        "#,
        slug
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

pub async fn update_post(
    pool: &PgPool,
    post_id: Uuid,
    update: &UpdatePost,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE posts
        SET
            title = $2,
            content = $3,
            updated_at = NOW()
        WHERE id = $1
        "#,
        post_id,
        update.title,
        update.content
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
        email,
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
