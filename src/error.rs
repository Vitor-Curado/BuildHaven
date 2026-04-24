use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Template error: {0}")]
    Template(#[from] askama::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Generic error")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Resource not found")]
    NotFound,

    #[error("Authentication error: unauthorized")]
    Unauthorized,

    #[error("Internal server error")]
    Internal,
}

// Type alias for convenience
pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        tracing::error!(
            error = %self,
            status = %status,
            "Application error"
        );

        (status, self.client_message()).into_response()
    }
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn client_message(&self) -> &'static str {
        match self {
            AppError::NotFound => "Not found",
            AppError::Unauthorized => "Unauthorized",
            _ => "Internal server error",
        }
    }
}
