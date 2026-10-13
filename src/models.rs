use serde::Deserialize;
use time::{Duration, OffsetDateTime};
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
    pub seo_title: String,
    pub meta_description: String,
    pub slug: String,
    pub content: String,
    pub status: String,
    pub views: i32,
    pub published_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Post {
    pub fn formatted_created_at(&self) -> String {
        self.created_at
            .format(&time::macros::format_description!(
                "[month repr:long] [day], [year] | [hour]:[minute]"
            ))
            .unwrap()
    }

    pub fn formatted_published_at(&self) -> Option<String> {
        self.published_at.map(|date| {
            date.format(&time::macros::format_description!(
                "[month repr:long] [day], [year] | [hour]:[minute]"
            ))
            .unwrap()
        })
    }

    pub fn formatted_updated_at(&self) -> String {
        self.updated_at
            .format(&time::macros::format_description!(
                "[month repr:long] [day], [year] | [hour]:[minute]"
            ))
            .unwrap()
    }

    pub fn time_since_publication(&self) -> Option<Duration> {
        self.published_at
            .map(|published_at| OffsetDateTime::now_utc() - published_at)
    }

    pub fn formatted_time_since_publication(&self) -> Option<String> {
        self.time_since_publication().map(Self::format_duration)
    }

    pub fn time_since_update(&self) -> Duration {
        OffsetDateTime::now_utc() - self.updated_at
    }

    pub fn formatted_time_since_update(&self) -> String {
        Self::format_duration(self.time_since_update())
    }

    pub fn time_since_creation(&self) -> Duration {
        OffsetDateTime::now_utc() - self.created_at
    }

    pub fn formatted_time_since_creation(&self) -> String {
        Self::format_duration(self.time_since_creation())
    }

    fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.whole_seconds().max(0);
        let days = total_seconds / 86400;
        let hours = (total_seconds % 86400) / 3600;
        let minutes = (total_seconds % 3600) / 60;

        let unit = |value: i64, singular: &str| {
            format!(
                "{value} {}",
                if value == 1 {
                    singular.to_string()
                } else {
                    format!("{singular}s")
                }
            )
        };

        match (days, hours, minutes) {
            (0, 0, minutes) => unit(minutes, "minute"),
            (0, hours, minutes) => format!("{}, {}", unit(hours, "hour"), unit(minutes, "minute")),
            (days, hours, minutes) => format!(
                "{}, {}, {}",
                unit(days, "day"),
                unit(hours, "hour"),
                unit(minutes, "minute")
            ),
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: OffsetDateTime,
    pub colour: String,
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

#[derive(Debug, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub seo_title: String,
    pub meta_description: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePost {
    pub title: String,
    pub seo_title: String,
    pub meta_description: String,
    pub content: String,
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
