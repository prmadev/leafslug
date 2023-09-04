//! http routes
pub mod health_check;
pub mod login;
use axum::Router;

/// a function that merges a list routes into one
pub fn merger(routes_given: Vec<Router>) -> Router
where
{
    routes_given
        .into_iter()
        .fold(Router::new(), axum::Router::merge)
}
