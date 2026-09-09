use crate::{
    error::{AppError, AppResult},
    repository::find_user_by_id,
    state::AppState,
};

use argon2::{
    Argon2,
    password_hash::{PasswordVerifier, phc::PasswordHash},
};
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use tower_sessions::Session;
use uuid::Uuid;

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub async fn require_auth(
    State(state): State<AppState>,
    session: Session,
    mut request: Request<Body>,
    next: Next,
) -> AppResult<Response> {
    let user_id = session
        .get::<Uuid>("user_id")
        .await?
        .ok_or(AppError::Unauthorized)?;

    let user = find_user_by_id(&state.db, user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
