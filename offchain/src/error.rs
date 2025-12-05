use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IPFS error: {0}")]
    IpfsError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Hash computation error: {0}")]
    HashError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Invalid CID: {0}")]
    InvalidCid(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::IpfsError(ref msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
            AppError::SerializationError(ref e) => (
                StatusCode::BAD_REQUEST,
                format!("Serialization error: {}", e),
            ),
            AppError::ValidationError(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::InternalError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::HashError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::IoError(ref e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("IO error: {}", e),
            ),
            AppError::HttpError(ref e) => (StatusCode::BAD_GATEWAY, format!("HTTP error: {}", e)),
            AppError::InvalidCid(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized(ref msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

// Helper to convert anyhow errors
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}
