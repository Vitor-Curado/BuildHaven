use crate::models::{NewPost, Post, Tag, UpdatePost, User};
use slug::slugify;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_post(pool: &PgPool, new_post: &NewPost) -> Result<Post, sqlx::Error> {
    let id = Uuid::now_v7();
    let slug = slugify(&new_post.title);

    let post = sqlx::query_file_as!(
        Post,
        "queries/posts/create_post.sql",
        id,
        new_post.title,
        new_post.seo_title,
        new_post.meta_description,
        slug,
        new_post.content
    )
    .fetch_one(pool)
    .await?;

    Ok(post)
}

pub async fn get_all_posts(pool: &PgPool) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_file_as!(Post, "queries/posts/get_all_posts.sql")
        .fetch_all(pool)
        .await
}

pub async fn get_published_posts(pool: &PgPool) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_file_as!(Post, "queries/posts/get_published_posts.sql", "published")
        .fetch_all(pool)
        .await
}

pub async fn get_posts_paginated(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_file_as!(Post, "queries/posts/get_posts_paginated.sql", limit, offset)
        .fetch_all(pool)
        .await
}

pub async fn get_post_by_id(pool: &PgPool, post_id: Uuid) -> Result<Option<Post>, sqlx::Error> {
    let post = sqlx::query_file_as!(Post, "queries/posts/get_post_by_id.sql", post_id)
        .fetch_optional(pool)
        .await?;

    Ok(post)
}

pub async fn get_posts_by_slug(pool: &PgPool, slug: String) -> Result<Option<Post>, sqlx::Error> {
    let post = sqlx::query_file_as!(Post, "queries/posts/get_posts_by_slug.sql", slug)
        .fetch_optional(pool)
        .await?;

    Ok(post)
}

pub async fn update_post(
    pool: &PgPool,
    post_id: Uuid,
    update: &UpdatePost,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_file!(
        "queries/posts/update_post.sql",
        post_id,
        update.title,
        update.seo_title,
        update.meta_description,
        update.content
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn delete_post(pool: &PgPool, post_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_file!("queries/posts/delete_post.sql", post_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn publish_post(pool: &PgPool, post_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_file!("queries/posts/publish_post.sql", post_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn unpublish_post(pool: &PgPool, post_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_file!("queries/posts/unpublish_post.sql", post_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn increment_post_views(pool: &PgPool, post_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_file!("queries/posts/increment_post_views.sql", post_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

// *** User-related stuff ***

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_file_as!(User, "queries/users/find_user_by_email.sql", email,)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn find_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_file_as!(User, "queries/users/find_user_by_id.sql", user_id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

// *** Tag-related stuff ***

pub async fn get_all_tags(pool: &PgPool) -> Result<Vec<Tag>, sqlx::Error> {
    sqlx::query_file_as!(Tag, "queries/tags/get_all_tags.sql")
        .fetch_all(pool)
        .await
}

pub async fn get_tag_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Tag>, sqlx::Error> {
    sqlx::query_file_as!(Tag, "queries/tags/get_tag_by_slug.sql", slug)
        .fetch_optional(pool)
        .await
}

pub async fn get_tags_for_post(pool: &PgPool, post_id: Uuid) -> Result<Vec<Tag>, sqlx::Error> {
    sqlx::query_file_as!(Tag, "queries/tags/get_tags_for_post.sql", post_id)
        .fetch_all(pool)
        .await
}

pub async fn get_published_posts_by_tag(
    pool: &PgPool,
    tag_slug: &str,
) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_file_as!(
        Post,
        "queries/posts/get_published_posts_by_tag.sql",
        tag_slug
    )
    .fetch_all(pool)
    .await
}
