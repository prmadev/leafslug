//! health check information

use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::{instrument, Level};

/// Simple health checker.
#[instrument(name = "Checking health")]
pub async fn health_check_simple() -> impl IntoResponse {
    tracing::event!(Level::INFO, "checked health successfully");

    println!("checked health successfully");
    Json(HealthCheckSimpleResp {
        message: " I am up!".to_owned(),
    })
}

/// response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HealthCheckSimpleResp {
    /// a mock message
    pub message: String,
}

/// router of /health_check/... end points
pub fn health_check_router() -> Router {
    Router::new().nest(
        "/health_check",
        Router::new().route("/simple", get(health_check_simple)),
    )
}
