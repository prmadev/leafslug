//! Leafslugs are cure.

use std::env::var_os;

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

    let db_user = var_os("DATABASE_USER")
        .ok_or("could not get DATABASE_USER")?
        .into_string()
        .map_err(|_e| "could not convert to string")?;

    let db_pass = var_os("DATABASE_PASS")
        .ok_or("could not get DATABASE_PASS")?
        .into_string()
        .map_err(|_e| "could not convert to string")?;

    let db_address = var_os("DATABASE_ADDRESS")
        .ok_or("could not get DATABASE_ADDRESS")?
        .into_string()
        .map_err(|_e| "could not convert to string")?;

    let db_name = var_os("DATABASE_NAME")
        .ok_or("could not get DATABASE_NAME")?
        .into_string()
        .map_err(|_e| "could not convert to string")?;

    let db_port = var_os("DATABASE_PORT")
        .ok_or("could not get DATABASE_PORT")?
        .into_string()
        .map_err(|_e| "could not convert to string")?;

    let db_url = format!("postgres://{db_user}:{db_pass}@{db_address}:{db_port}/{db_name}");
    _ = db_url;

    Ok(())
}
