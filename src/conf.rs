//! configuration management

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

/// parses and retrieves configuration
pub fn conf() -> anyhow::Result<Config> {
    let config: Config = Figment::new()
        .join(Toml::file("leafslug.toml"))
        .merge(Env::prefixed("LEAFSLUG_"))
        .extract()?;

    Ok(config)
}

/// configuration related to db
#[derive(Deserialize, Debug)]
pub struct DBConf {
    /// name of the databse
    pub name: String,
    /// name of the database user
    pub user: String,
    /// password of the database
    pub pass: String,
    /// host name of the database
    pub host: String,
    /// port number of the database
    pub port: u16,
}

/// configurations related to the main rest service
#[derive(Deserialize, Debug)]
pub struct RESTConf {
    /// Host to use
    /// It should look like this: `0.0.0.0`
    pub host: String,
    /// Port  number to use
    pub port: u16,
}

#[derive(Deserialize, Debug)]
/// configuration structure
pub struct Config {
    /// configurations related to db
    pub db: DBConf,
    /// configurations related to REST server
    pub rest: RESTConf,
}
