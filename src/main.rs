//! Leafslugs are cute.

use axum::Router;

use leafslug::{conf, health_check::health_check_router, http, routes_merger};

use tokio::select;
use tracing::info;
use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, Registry};

/// Lesly is a cute leafslug
const LESLY: &str = "\n

                 ▓  ▓    ▓▓▓    ▓   ▓  ▓▓    ▓▓
            ▓▓  ▓▓ ▓▓▓ ▓▓▓▓▓▓▓ ▓▓▓ ▓▓ ▓▓▓▓  ▓▓▓▓▓ ▓▓
     ▓ ▓    ▓▓▓ ▓▓ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  ▓▓▓▓▓▓▓▓▓▓▓
    ▓▓▓▓▓▓▓  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
   ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓     ▓▓
  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓████▓▓▓▓████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    ▓▓▓▓▓▓▓▓▓▓▓▓▓█████████████████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
  ▓▓▓▓▓▓▓▓▓▓█████████████████████████████████▓▓▓▓▓▓▓▓▓▓▓▓
 ▓▓▓▓▓▓▓▓█████████████████ ████ ████████████████▓▓▓▓▓▓▓▓▓▓▓▓
   ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██████████████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
 ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██████████████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██▓▓██████████▓▓██▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██████████████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
██████████████████████████████████████████████████████████████
";

#[tokio_macros::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{LESLY}");

    //
    //  Setting Up Logging
    //

    let formatting_layer = tracing_bunyan_formatter::BunyanFormattingLayer::new(
        "Leafslug".to_owned(),
        std::io::stdout,
    );

    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    //
    //  Setting Up Configurations
    //

    let configurations = conf()?;
    info!("{:#?}", &configurations);

    tracing::info!("starting application");

    //
    // Setting Up Rest Routes
    //

    let api_v1_routes = {
        let v1 = Router::new().nest("/v1", routes_merger(vec![health_check_router()]).await);
        Router::new().nest("/api", v1)
    };

    //
    // Serving REST
    //

    let rest_handler = http::http_serve(
        configurations.rest.host,
        configurations.rest.port,
        routes_merger(vec![api_v1_routes]).await,
    );

    //
    // Checking For Errors From Failure Of A Service
    //

    select! {
        res = rest_handler => {
           res?;
        }
    }

    Ok(())
}
