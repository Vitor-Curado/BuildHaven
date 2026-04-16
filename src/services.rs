use crate::{auth::AuthService, models::Post, repository};
use sqlx::PgPool;

pub struct Services {
    pub db: PgPool,
    pub auth: AuthService,
}

pub async fn list_posts(pool: &PgPool) -> Result<Vec<Post>, sqlx::Error> {
    let posts = repository::get_all_posts(pool).await?;

    // future logic goes here
    // filtering, transformation, validation, etc.

    Ok(posts)
}
