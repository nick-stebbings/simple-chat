use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
  pub host: String,
  pub port: u16
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    Ok(envy::from_env::<Config>()?)
}
