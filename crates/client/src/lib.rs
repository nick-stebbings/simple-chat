pub mod config;
use tokio::net::TcpStream;

#[tokio::main]
pub async fn run(config_values: &config::Config) -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("{}:{}", config_values.host, config_values.port);
    let mut stream = TcpStream::connect(&address).await?;
    println!("Connected to the server at {}", address);

    Ok(())
}
