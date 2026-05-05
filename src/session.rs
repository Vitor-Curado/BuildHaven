use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{config::Config, models::Session};

pub async fn create_session(
    pool: &PgPool,
    user_id: Uuid,
    config: &Config,
) -> Result<Session, sqlx::Error> {
    let session_id = Uuid::now_v7();
    let now = Utc::now();
    let expires_at = now + Duration::hours(config.session.duration_hours); // Session valid for 24 hours

    sqlx::query!(
        "INSERT INTO sessions (id, user_id, created_at, expires_at) VALUES ($1, $2, $3, $4)",
        session_id,
        user_id,
        now,
        expires_at
    )
    .execute(pool)
    .await?;

    Ok(Session {
        id: session_id,
        user_id,
        created_at: now,
        expires_at,
    })
}

pub async fn get_session_by_id(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<Option<Session>, sqlx::Error> {
    let record = sqlx::query_as!(
        Session,
        "SELECT id, user_id, created_at, expires_at FROM sessions WHERE id = $1 AND expires_at > NOW()",
        session_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn delete_session(pool: &PgPool, session_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM sessions WHERE id = $1", session_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_sessions_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM sessions WHERE user_id = $1", user_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_expired_sessions(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM sessions WHERE expires_at < NOW()")
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn refresh_session_expiry(pool: &PgPool, session_id: Uuid) -> Result<(), sqlx::Error> {
    let new_expires_at = Utc::now() + Duration::hours(24);

    sqlx::query!(
        "UPDATE sessions SET expires_at = $1 WHERE id = $2",
        new_expires_at,
        session_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// Todo: Add functions for session management, such as refreshing expiry, deleting sessions by user ID, etc.
