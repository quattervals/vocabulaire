use thiserror::Error;
use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};

#[derive(Debug, PartialEq, Error)]
pub enum ApiError {
    #[error("nix")]
    BadRequest(String),
    #[error("nix")]
    InternalServerError(String),
    #[error("nix")]
    NotFound(String),
    #[error("nix")]
    InvalidData(String),
    #[error("nix")]
    Unknown(String),
    #[error("nix")]
    Conflict(String),
    #[error("nix")]
    ValidationError(Vec<String>),
}


/// Automatically convert ApiErrors to external ResponseError
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) | ApiError::InvalidData(error) => {
                HttpResponse::BadRequest().json(error)
            }
            ApiError::NotFound(message) => HttpResponse::NotFound().json(message),
            ApiError::ValidationError(errors) => {
                HttpResponse::UnprocessableEntity().json(&errors.to_vec())
            }
            ApiError::InternalServerError(error) => HttpResponse::Unauthorized().json(error),
            ApiError::Conflict(error) => HttpResponse::Conflict().json(error),
            ApiError::Unknown(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
