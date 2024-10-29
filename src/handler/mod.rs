use actix_web::http::StatusCode;
use thiserror::Error;
use webauthn_rs::prelude::WebauthnError;

pub(crate) mod auth;

/**
Type alias for Errors that implement [actix_web::ResponseError] through [Error]
*/
type WebResult<T> = Result<T, Error>;

/**
Unified errors for simpler Responses
*/
#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("Unknown webauthn error")]
    Unknown(WebauthnError),
    #[error("Corrupt session")]
    CorruptSession,
    #[error("Bad request")]
    BadRequest(#[from] WebauthnError),
    #[error("User not found")]
    UserNotFound,
    #[error("User has no credentials")]
    UserHasNoCredentials,
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
