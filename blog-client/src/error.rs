use std::{env::VarError, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Input-output error: {0}")]
    Io(#[from] io::Error),

    #[error("Client error: {0}")]
    ReqwestClientError(#[from] reqwest::Error),

    #[error("Invalid request parameters: {0}")]
    InvalidRequestError(String),

    #[error("You have to authorized")]
    UnAuthorizedError,

    #[error("User not found error")]
    UserNotFoundError,

    #[error("User already exist")]
    UserAlreadyExistsError,

    #[error("Invalid credentials")]
    InvalidCredentialsError,

    #[error("Post not found")]
    PostNotFoundError,

    #[error("Required item not found")]
    ItemNotFoundError,

    #[error("Forbidden to change post")]
    Forbidden,

    #[error("Config var reading error: {0}")]
    VarReadingError(#[from] VarError),

    #[error("Offset and limit must be > 0 and < 30")]
    PaganationError,

    #[error("Server error: {0}")]
    InternalServerError(#[from] Box<dyn std::error::Error>),

    #[error("Server return an error: {0}")]
    ServerError(String),

    #[error("GRPC transport error: {0}")]
    GRPCTransportError(#[from] tonic::transport::Error),

    #[error("GRPC status error: {0}")]
    GRPCStatusError(#[from] tonic::Status),
}