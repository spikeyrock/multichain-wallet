use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Invalid word count: {0}. Must be 12, 15, 18, 21, or 24")]
    InvalidWordCount(u32),

    #[error("Invalid language: {0}")]
    InvalidLanguage(String),

    #[error("Invalid mnemonic phrase")]
    InvalidMnemonic,

    #[error("Invalid derivation path: {0}")]
    InvalidDerivationPath(String),

    #[error("Cryptographic operation failed: {0}")]
    CryptoError(String),

    #[error("Internal server error")]
    InternalError,

    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = json!({
            "error": {
                "code": status_code.as_u16(),
                "message": self.to_string(),
                "type": self.error_type(),
            }
        });

        HttpResponse::build(status_code).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InvalidWordCount(_) => StatusCode::BAD_REQUEST,
            ApiError::InvalidLanguage(_) => StatusCode::BAD_REQUEST,
            ApiError::InvalidMnemonic => StatusCode::BAD_REQUEST,
            ApiError::InvalidDerivationPath(_) => StatusCode::BAD_REQUEST,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::CryptoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ApiError {
    fn error_type(&self) -> &str {
        match self {
            ApiError::InvalidWordCount(_) => "INVALID_WORD_COUNT",
            ApiError::InvalidLanguage(_) => "INVALID_LANGUAGE",
            ApiError::InvalidMnemonic => "INVALID_MNEMONIC",
            ApiError::InvalidDerivationPath(_) => "INVALID_DERIVATION_PATH",
            ApiError::BadRequest(_) => "BAD_REQUEST",
            ApiError::CryptoError(_) => "CRYPTO_ERROR",
            ApiError::InternalError => "INTERNAL_ERROR",
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;