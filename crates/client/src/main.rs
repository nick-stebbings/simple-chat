use clap::{arg, command, Parser};
use client::run;
use common::config::get_config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "anon")]
    pub username: String,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_values = get_config()?;
    let address = format!("{}:{}", config_values.host, config_values.port);

    let args = Args::parse();
    println!("Hello, {}!", args.username);

    run(address).await
}
