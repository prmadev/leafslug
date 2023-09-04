//! health check information

use axum::{http::status, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::{instrument, Level};

/// Simple health checker.
#[instrument(name = "Checking health")]
pub async fn handler_simple() -> impl IntoResponse {
    tracing::event!(Level::INFO, "checked health successfully");

    println!("checked health successfully");
    (
        status::StatusCode::OK,
        Json(Response {
            message: " I am up!".to_owned(),
        }),
    )
}

/// response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    /// a mock message
    pub message: String,
}

/// router of health check end points
pub fn router() -> Router {
    Router::new().nest(
        "/health_check",
        Router::new().route("/simple", get(handler_simple)),
    )
}
