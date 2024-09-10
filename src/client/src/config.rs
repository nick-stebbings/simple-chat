use std::error::Error;

use clap::{arg, command, Parser};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    Ok(envy::from_env::<Config>()?)
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "anon")]
    pub username: String,
}
