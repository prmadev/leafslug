//! server component

use axum::Router;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use thiserror::Error;

pub mod routes;
pub use routes::*;

/// Runner for the REST server.
pub async fn http_serve(app: Router) -> Result<(), ServerErr> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8095);

    let s = axum::Server::try_bind(&addr)
        .map_err(|x| ServerErr::CouldNotBind(addr, x.message().to_string()))?
        .serve(app.into_make_service());
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    let grace = s.with_graceful_shutdown(async {
        rx.await.map_err(|_| ServerErr::RecieverDropped).ok();
    });

    if let Err(e) = grace.await {
        tracing::error!("{e}");
    }

    tx.send(()).map_err(|_| ServerErr::SenderFailed)
}

/// Errors related to running the server.
#[derive(Error, Debug)]
pub enum ServerErr {
    /// Error related to binding to sepcific address.
    #[error("could not bind to address {0}: {1}")]
    CouldNotBind(SocketAddr, String),

    /// Error related to the channel made for handling the shutdown gracefully.
    #[error("somehting internal broke when handling shutdown gracefully on the receiver side")]
    RecieverDropped,

    /// Error related to the channel made for handling the shutdown gracefully.
    #[error("somehting internal broke when handling errors gracefully on the sender side")]
    SenderFailed,
}
