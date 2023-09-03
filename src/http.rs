//! server component

use axum::Router;
use thiserror::Error;

pub mod routes;
pub use routes::*;

/// Runner for the REST server.
pub async fn http_serve(host: String, port: u16, app: Router) -> Result<(), ServerErr> {
    // let addr = SocketAddr::new(IpAddr::V4(a));

    // let addr = url
    let address = format!("{host}:{port}");
    let addr = std::net::TcpListener::bind(&address)
        .map_err(|x| ServerErr::CouldNotBind(address, x.to_string()))?;

    let s = axum::Server::from_tcp(addr)
        .map_err(|x| ServerErr::CouldNotConvertBinding(x.message().to_string()))?
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
