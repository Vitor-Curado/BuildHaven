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
    Config(#[from] envy::Error),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Resource not found")]
    NotFound,

    #[error("Authentication error: unauthorized")]
    Unauthorized,

    #[error(transparent)]
    Session(#[from] tower_sessions::session::Error),

    #[error("Asset manifest is missing required asset: {0}")]
    MissingAsset(&'static str),

    #[error("Invalid UTF-8 in SVG")]
    InvalidSvgUtf8(#[source] std::str::Utf8Error),

    #[error("Failed to parse SVG: {0}")]
    SvgParse(String),
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
            AppError::Session(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::MissingAsset(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidSvgUtf8(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SvgParse(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn client_message(&self) -> &'static str {
        match self {
            AppError::NotFound => errors::NOT_FOUND,
            AppError::Unauthorized => errors::UNAUTHORIZED,
            AppError::BadRequest(_) => errors::BAD_REQUEST,
            AppError::Config(_) => errors::CONFIG_ERROR,
            AppError::MissingAsset(_) => errors::MISSING_ASSET,
            AppError::Session(_) => errors::SESSION_ERROR,
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
            AppError::MissingAsset(_) => "MISSING_ASSET",
            AppError::InvalidSvgUtf8(_) => "INVALID_SVG_UTF8",
            AppError::SvgParse(_) => "SVG_PARSE_ERROR",
            AppError::Session(_) => "SESSION_ERROR",
        }
    }
}
