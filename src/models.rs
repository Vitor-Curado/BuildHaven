use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Post {
    pub fn formatted_date(&self) -> String {
        self.created_at
            .date()
            .format(&time::macros::format_description!("[year]-[month]-[day]"))
            .unwrap()
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}

pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: Vec<u8>,
    pub created_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
}

pub struct CreatedSession {
    pub session: Session,
    pub token: String,
}
