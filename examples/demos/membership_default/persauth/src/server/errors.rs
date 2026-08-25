use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Duplicate value: {0}")]
    DuplicateValue(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Invalid ID")]
    BadId,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Process error: {0}")]
    ProcessError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Decryption error: {0}")]
    DecryptError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Configuration error: {0}")]
    FaultySetup(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Pool error: {0}")]
    PoolError(String),
}

/// Error response format that matches ApiResponse for frontend compatibility
#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    message: String,
    error: Option<String>,
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            ServiceError::BadId => (StatusCode::BAD_REQUEST, "bad_id", "Invalid ID".to_string()),
            ServiceError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            ServiceError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.clone()),
            ServiceError::Conflict(msg) => (StatusCode::CONFLICT, "conflict", msg.clone()),
            ServiceError::DuplicateValue(msg) => (StatusCode::CONFLICT, "duplicate", msg.clone()),
            ServiceError::ProcessError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "process_error", msg.clone())
            }
            ServiceError::DatabaseError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "database_error", msg.clone())
            }
            ServiceError::InternalServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg.clone())
            }
            ServiceError::AuthenticationError(msg) => {
                (StatusCode::UNAUTHORIZED, "auth_error", msg.clone())
            }
            ServiceError::FaultySetup(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "setup_error", msg.clone())
            }
            ServiceError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, "unauthorized", msg.clone())
            }
            ServiceError::Forbidden(msg) => {
                (StatusCode::FORBIDDEN, "forbidden", msg.clone())
            }
            ServiceError::DecryptError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "decrypt_error", msg.clone())
            }
            ServiceError::PoolError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "pool_error", msg.clone())
            }
        };

        let body = Json(ErrorResponse {
            success: false,
            message,
            error: Some(error_type.to_string()),
        });

        (status, body).into_response()
    }
}

impl From<tokio_postgres::Error> for ServiceError {
    fn from(error: tokio_postgres::Error) -> Self {
        let error_msg = format!("{}", error);
        log::error!("Database error: {}", error_msg);
        // Include more detail if available
        if let Some(db_error) = error.as_db_error() {
            let detailed = format!(
                "{} (code: {}, detail: {:?}, hint: {:?})",
                db_error.message(),
                db_error.code().code(),
                db_error.detail(),
                db_error.hint()
            );
            log::error!("DB error details: {}", detailed);
            return ServiceError::DatabaseError(detailed);
        }
        ServiceError::DatabaseError(error_msg)
    }
}

impl From<deadpool_postgres::PoolError> for ServiceError {
    fn from(error: deadpool_postgres::PoolError) -> Self {
        let error_msg = format!("{}", error);
        log::error!("Pool error: {}", error_msg);
        ServiceError::PoolError(error_msg)
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(error: std::io::Error) -> Self {
        ServiceError::InternalServerError(error.to_string())
    }
}

impl From<anyhow::Error> for ServiceError {
    fn from(error: anyhow::Error) -> Self {
        ServiceError::InternalServerError(error.to_string())
    }
}

impl From<std::str::Utf8Error> for ServiceError {
    fn from(err: std::str::Utf8Error) -> ServiceError {
        ServiceError::FaultySetup(err.to_string())
    }
}

impl From<std::num::ParseIntError> for ServiceError {
    fn from(err: std::num::ParseIntError) -> ServiceError {
        ServiceError::FaultySetup(err.to_string())
    }
}
