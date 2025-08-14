use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::{self, Response};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("validation failed")]
    Validation(#[from] validator::ValidationErrors),

    #[error("Invalid request format: {0}")]
    RequestFormat(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("An internal database error occurred")]
    Database(#[from] sqlx::Error),

    #[error("Failed to get config")]
    Config(#[from] config::ConfigError),

    #[error("Multipart request error")]
    Multipart(#[from] axum::extract::multipart::MultipartError),

    #[error("Failed to parse JSON")]
    JsonParse(#[from] serde_json::Error),

    #[error("Clock may have gone backwards")]
    TimeError(#[from] std::time::SystemTimeError),

    #[error("An internal server error occurred")]
    Internal,
    // #[error("Failed to generate QR code image")]
    // TotpQrGeneration(String),

    // #[error("An internal error occurred during password hashing")]
    // Hashing(#[from] HashingError),

    // #[error("Failed to generate unique ID")]
    // IdGeneration(#[from] SnowflakeError),

    // #[error("A TOTP secret is malformed")]
    // TotpSecretMalformed(#[from] totp_rs::SecretParseError),

    // #[error("Failed to generate TOTP URL for QR code")]
    // TotpUrl(#[from] totp_rs::TotpUrlError),

    // #[error("JWT operation failed")]
    // Jwt(#[from] JwtError),

    // #[error("Redis operation failed")]
    // Redis(#[from] redis::RedisError),

    // #[error("Redis connection pool error")]
    // RedisPool(#[from] bb8::RunError<redis::RedisError>),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message, details) = match self {
            AppError::Validation(err) => {
                let details = json!(err.field_errors());
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "Validation failed".to_string(),
                    Some(details),
                )
            }
            AppError::RequestFormat(msg) => (StatusCode::BAD_REQUEST, msg, None),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg, None),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg, None),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg, None),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg, None),
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                    None,
                )
            }
            AppError::Config(err) => {
                tracing::error!("Config getter error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                    None,
                )
            }
            AppError::Multipart(err) => {
                tracing::warn!("Multipart request error: {:?}", err);
                (
                    StatusCode::BAD_REQUEST,
                    "Invalid multipart form data".to_string(),
                    None,
                )
            }
            AppError::JsonParse(err) => {
                tracing::error!("Failed to parse JSON: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                    None,
                )
            }
            AppError::TimeError(err) => {
                tracing::error!("Failed to generate TOTP QR code: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                    None,
                )
            }
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred".to_string(),
                None,
            ),
        };
        (status, Json(ErrorResponse { message, details })).into_response()
    }
}
