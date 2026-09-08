// investigate tower-sessions

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{Duration, Utc};
use rand::RngCore;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::Config,
    models::{CreatedSession, Session},
};

pub async fn create_session(
    pool: &PgPool,
    user_id: Uuid,
    config: &Config,
) -> Result<CreatedSession, sqlx::Error> {
    let session_id = Uuid::now_v7();
    let now = Utc::now();
    let expires_at = now + Duration::hours(config.session.duration_hours);

    // Generate a 256-bit session token
    let mut token_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut token_bytes);

    // Encode the token for a safe use in cookies and URLs
    let token = URL_SAFE_NO_PAD.encode(&token_bytes);

    // Store only the hash in the database for security
    let token_hash = hash_session_token(&token);

    sqlx::query!(
        "INSERT INTO sessions (id, user_id, token_hash, created_at, expires_at) VALUES ($1, $2, $3, $4, $5)",
        session_id,
        user_id,
        token_hash,
        now,
        expires_at
    )
    .execute(pool)
    .await?;

    Ok(CreatedSession {
        session: Session {
            id: session_id,
            user_id,
            token_hash,
            created_at: now,
            expires_at,
        },
        token,
    })
}

pub async fn get_session_by_token_hash(
    pool: &PgPool,
    token_hash: &[u8],
) -> Result<Option<Session>, sqlx::Error> {
    let record = sqlx::query_as!(
        Session,
        "SELECT id, user_id, token_hash, created_at, expires_at FROM sessions WHERE token_hash = $1 AND expires_at > NOW()",
        token_hash
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

pub async fn delete_expired_sessions(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM sessions
        WHERE expires_at < NOW()
        "#
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn refresh_session_expiry(
    pool: &PgPool,
    session_id: Uuid,
    config: &Config,
) -> Result<(), sqlx::Error> {
    let new_expires_at = Utc::now() + Duration::hours(config.session.duration_hours);

    sqlx::query!(
        "UPDATE sessions SET expires_at = $1 WHERE id = $2",
        new_expires_at,
        session_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub fn hash_session_token(token: &str) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hasher.finalize().to_vec()
}
