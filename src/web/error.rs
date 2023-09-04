//! Error that may come out of the REST.

use axum::{http::status, response::IntoResponse};
use serde::Serialize;

/// The default result type of the web interface.
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Clone)]
/// Error type for web service.
pub enum Error {
    /// 400: Bad Request
    #[error("invalid input, at: {} , because: {}", .0.at, .0.why)]
    BadRequest(GenericResponsePayload),
    /// 500: internal server error
    #[error("internal server error, at: {} , because: {}", .0.at, .0.why)]
    GenericInternalServerError(GenericResponsePayload),
    /// : invalid username and password
    #[error("invalid username and password.")]
    InvalidUsernameOrPassword,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::BadRequest(er) => {
                (status::StatusCode::BAD_REQUEST, axum::Json(er)).into_response()
            }
            Self::GenericInternalServerError(er) => {
                (status::StatusCode::INTERNAL_SERVER_ERROR, axum::Json(er)).into_response()
            }
            Self::InvalidUsernameOrPassword => (
                status::StatusCode::NOT_ACCEPTABLE,
                "Username or password are not valid!",
            )
                .into_response(),
        }
    }
}

/// A very simple struct that contains generic information useful for
/// debugging and addressing problems with the code.
#[derive(Debug, Clone, Serialize)]
pub struct GenericResponsePayload {
    at: String,
    why: String,
}
