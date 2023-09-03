//! server component

use axum::Router;
use thiserror::Error;

pub mod routes;
pub use routes::*;

/// Runner for the REST server.
///
///
/// # Errors
///
/// This function will return an error if it fails to bind or fails to parse the related bindings.
pub async fn serve(host: String, port: u16, app: Router) -> Result<(), Error> {
    // let addr = url
    let address = format!("{host}:{port}");
    let addr = std::net::TcpListener::bind(&address)
        .map_err(|e| Error::CouldNotBind(address, e.to_string()))?;

    let s = axum::Server::from_tcp(addr)
        .map_err(|e| Error::CouldNotConvertBinding(e.message().to_string()))?
        .serve(app.into_make_service());
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    let grace = s.with_graceful_shutdown(async {
        rx.await.map_err(|_e| Error::RecieverDropped).ok();
    });

    if let Err(e) = grace.await {
        tracing::error!("{e}");
    }

    tx.send(()).map_err(|_e| Error::SenderFailed)
}

/// Errors related to running the server.
#[derive(Error, Debug)]
pub enum Error {
    /// Error related to binding to sepcific address.
    #[error("could not bind to address {0}: {1}")]
    CouldNotBind(String, String),

    /// Error related to binding to sepcific address.
    #[error("could not bind to address: {0}")]
    CouldNotConvertBinding(String),

    /// Error related to the channel made for handling the shutdown gracefully.
    #[error("somehting internal broke when handling shutdown gracefully on the receiver side")]
    RecieverDropped,

    /// Error related to the channel made for handling the shutdown gracefully.
    #[error("somehting internal broke when handling errors gracefully on the sender side")]
    SenderFailed,
}
