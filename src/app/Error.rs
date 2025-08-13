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
    Database(#[from] sea_orm::DbErr),

    #[error("An internal error occurred during password hashing")]
    Hashing(#[from] HashingError),

    #[error("Failed to generate unique ID")]
    IdGeneration(#[from] SnowflakeError),

    #[error("Failed to get config")]
    Config(#[from] ConfigError),

    #[error("JWT operation failed")]
    Jwt(#[from] JwtError),

    #[error("Redis operation failed")]
    Redis(#[from] redis::RedisError),

    #[error("Redis connection pool error")]
    RedisPool(#[from] bb8::RunError<redis::RedisError>),

    #[error("Multipart request error")]
    Multipart(#[from] axum::extract::multipart::MultipartError),

    #[error("Failed to parse JSON")]
    JsonParse(#[from] serde_json::Error),

    #[error("A TOTP secret is malformed")]
    TotpSecretMalformed(#[from] totp_rs::SecretParseError),

    #[error("Failed to generate TOTP URL for QR code")]
    TotpUrl(#[from] totp_rs::TotpUrlError),

    #[error("Failed to generate QR code image")]
    TotpQrGeneration(String),

    #[error("Clock may have gone backwards")]
    TimeError(#[from] std::time::SystemTimeError),

    #[error("An internal server error occurred")]
    Internal,
}
