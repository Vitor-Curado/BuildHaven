use crate::{
    error::{AppError, AppResult},
    state::AppState,
};

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use axum_extra::extract::cookie::CookieJar;
use rand_core::OsRng;

pub struct AuthService {
    argon2: Argon2<'static>,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    pub fn hash_password(&self, password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);

        self.argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("argon2 hashing failed")
            .to_string()
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> bool {
        let parsed_hash = match PasswordHash::new(hash) {
            Ok(h) => h,
            Err(_) => return false,
        };

        self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

pub async fn require_auth(
    State(state): State<AppState>,
    jar: CookieJar,
    request: Request<Body>,
    next: Next,
) -> AppResult<Response> {
    if let Some(token) = jar.get("auth_token") {
        if state
            .ctx
            .services
            .auth
            .verify_password(token.value(), "expected_hash")
        {
            return Ok(next.run(request).await);
        }
    }

    Err(AppError::Unauthorized)
}