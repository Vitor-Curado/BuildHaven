use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Session;

pub async fn create_session(pool: &PgPool, user_id: Uuid) -> Result<Session, sqlx::Error> {
    let session_id = Uuid::new_v4();
    let now = Utc::now();
    let expires_at = now + Duration::hours(24); // Session valid for 24 hours

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
