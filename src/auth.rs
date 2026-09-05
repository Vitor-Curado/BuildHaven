use crate::{
    error::{AppError, AppResult},
    repository::find_user_by_id,
    session::get_session_by_id,
    state::AppState,
};

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use axum_extra::extract::cookie::CookieJar;

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
    jar: CookieJar,
    mut request: Request<Body>,
    next: Next,
) -> AppResult<Response> {
    let cookie = jar.get("session_id").ok_or(AppError::Unauthorized)?;

    let session_id = cookie.value().parse().map_err(|_| AppError::Unauthorized)?;

    let session = get_session_by_id(&state.db, session_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let user = find_user_by_id(&state.db, session.user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}