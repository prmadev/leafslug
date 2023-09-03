//! Leafslugs are cute.

use axum::Router;
use leafslug::{health_check::health_check_router, http, routes_merger};
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
    let formatting_layer = tracing_bunyan_formatter::BunyanFormattingLayer::new(
        "Leafslug".to_owned(),
        std::io::stdout,
    );
    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    // let db_user = var_os("DATABASE_USER")
    //     .ok_or("could not get DATABASE_USER")?
    //     .into_string()
    //     .map_err(|_e| "could not convert to string")?;

    // let db_pass = var_os("DATABASE_PASS")
    //     .ok_or("could not get DATABASE_PASS")?
    //     .into_string()
    //     .map_err(|_e| "could not convert to string")?;

    // let db_address = var_os("DATABASE_ADDRESS")
    //     .ok_or("could not get DATABASE_ADDRESS")?
    //     .into_string()
    //     .map_err(|_e| "could not convert to string")?;

    // let db_name = var_os("DATABASE_NAME")
    //     .ok_or("could not get DATABASE_NAME")?
    //     .into_string()
    //     .map_err(|_e| "could not convert to string")?;

    // let db_port = var_os("DATABASE_PORT")
    //     .ok_or("could not get DATABASE_PORT")?
    //     .into_string()
    //     .map_err(|_e| "could not convert to string")?;

    // let db_url = format!("postgres://{db_user}:{db_pass}@{db_address}:{db_port}/{db_name}");
    // _ = db_url;
    tracing::info!("starting application");
    let api_v1_routes = {
        let v1 = Router::new().nest("/v1", routes_merger(vec![health_check_router()]).await);
        Router::new().nest("/api", v1)
    };

    http::http_serve(routes_merger(vec![api_v1_routes]).await).await?;
    Ok(())
}
