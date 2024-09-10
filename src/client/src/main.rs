mod config;

use clap::Parser;
use config::{get_config, Args};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Hello, {}!", args.username);

    let config_values = get_config()?;
    let address = format!("{}:{}", config_values.host, config_values.port);
    let mut stream = TcpStream::connect(&address).await?;
    println!("Connected to the server at {}", address);

    Ok(())
}
