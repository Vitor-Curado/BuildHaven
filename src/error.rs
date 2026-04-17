use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Template error")]
    Template(#[from] askama::Error),

    #[error("Resource not found")]
    NotFound,

    #[error("Authentication error: unauthorized")]
    Unauthorized,

    #[error("Internal server error")]
    Internal
}

// Type alias for convenience
pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };

        tracing::error!("Application error: {:?}", self);

        let message = match self {
            AppError::NotFound => "Not found",
            _ => "Internal server error",
        };

        (status, message).into_response()
    }
}
