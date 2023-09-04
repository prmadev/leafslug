//! Login handler for the website

use crate::web::error::{Error, Result};
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::{instrument, Level};

/// Handler for login.
#[instrument(name = "login")]
pub async fn handler(Json(payload): Json<Request>) -> Result<Json<Response>> {
    // TODO: authentication logic
    if payload.username != "alphauser" || payload.password != "alphapass" {
        tracing::event!(
            Level::ERROR,
            r#"invalid user name and password were given: {:#?}"#,
            payload
        );
        Err(Error::InvalidUsernameOrPassword)
    } else {
        Ok(Json(Response { success: true }))
    }
}

/// response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    /// username of the user
    pub username: String,

    /// hashed password of the user
    pub password: String,
}
/// response
#[derive(Debug, Clone, Deserialize, Serialize, Copy)]
pub struct Response {
    /// status of the response
    pub success: bool,
}

/// router of health check end points
pub fn router() -> Router {
    Router::new().nest("/login", Router::new().route("/simple", post(handler)))
}
