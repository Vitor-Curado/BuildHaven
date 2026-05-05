use crate::constants::errors;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(serde::Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: &'static str,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Template error: {0}")]
    Template(#[from] askama::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Generic error")]
    Other {
        message: &'static str,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

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
        let body = ErrorResponse {
            code: self.code(),
            message: self.client_message(),
        };

        tracing::error!(
            error.display = %self,
            error.debug = ?self,
            status = %status,
            "Application error"
        );

        (status, axum::Json(body)).into_response()
    }
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Serialization(_) => StatusCode::BAD_REQUEST,

            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Other { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn client_message(&self) -> &'static str {
        match self {
            AppError::NotFound => errors::NOT_FOUND,
            AppError::Unauthorized => errors::UNAUTHORIZED,
            AppError::BadRequest(_) => errors::BAD_REQUEST,
            _ => errors::INTERNAL,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            AppError::NotFound => "NOT_FOUND",
            AppError::Unauthorized => "UNAUTHORIZED",
            AppError::Database(_) => "DB_ERROR",
            AppError::Template(_) => "TEMPLATE_ERROR",
            AppError::Io(_) => "IO_ERROR",
            AppError::Serialization(_) => "SERDE_ERROR",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::Config(_) => "CONFIG_ERROR",
            AppError::Internal => "INTERNAL",
            AppError::Other { .. } => "UNKNOWN",
        }
    }
}
