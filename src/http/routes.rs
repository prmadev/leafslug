//! http routes
pub mod health_check;
use axum::Router;

/// a function that merges a list routes into one
pub async fn routes_merger(routes_given: Vec<Router>) -> Router
where
{
    routes_given
        .into_iter()
        .fold(Router::new(), |accu, item| accu.merge(item))
}
