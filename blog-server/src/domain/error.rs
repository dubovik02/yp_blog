use std::{env::VarError, io, net::AddrParseError};

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Input-output error: {0}")]
    Io(#[from] io::Error),

    #[error("User not found error")]
    UserNotFoundError,

    #[error("User already exist")]
    UserAlreadyExistsError,

    #[error("Invalid credentials")]
    InvalidCredentialsError,

    #[error("Post not found")]
    PostNotFoundError,

    #[error("Forbidden to change post")]
    Forbidden,

    #[error("Sqlx error has happend: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Migration error has happend: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("Coding (encoding) error: {0}")]
    CodingEncodingError(#[from] jsonwebtoken::errors::Error),

    #[error("Config var reading error: {0}")]
    VarReadingError(#[from] VarError),

    #[error("ENV-file reading error: {0}")]
    ENVLoadError(#[from] dotenvy::Error),

    #[error("Adress parse error: {0}")]
    AdressParseError(#[from] AddrParseError),

    #[error("Tonic transport error: {0}")]
    TonicTransportError(#[from] tonic::transport::Error),

    #[error("Internal Server error: {0}")]
    InternalServerError(#[from] Box<dyn std::error::Error>),

    #[error("Hashing pass error: {0}")]
    HashPassError(#[from] argon2::password_hash::Error),

    #[error("Hash params error: {0}")]
    HashParamsError(#[from] argon2::Error),

    #[error("Token is incorrect: {0}")]
    TokenError(#[from] actix_web::error::Error),

    #[error("Offset and limit must be > 0 and < 30")]
    PaganationError,
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            ServerError::PaganationError => StatusCode::BAD_REQUEST,
            ServerError::UserAlreadyExistsError => StatusCode::CONFLICT,
            ServerError::UserNotFoundError => StatusCode::NOT_FOUND,
            ServerError::PostNotFoundError => StatusCode::NOT_FOUND,
            ServerError::InvalidCredentialsError => StatusCode::UNAUTHORIZED,
            ServerError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::Forbidden => StatusCode::FORBIDDEN,
            _ => StatusCode::BAD_REQUEST
        };
        
        HttpResponse::build(status).json(serde_json::json!({
            "error": self.to_string(),
            "status": status.as_u16(),
        }))
    }
} 