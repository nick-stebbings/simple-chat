mod config;

use tokio::net::TcpStream;

use config::get_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_values = get_config()?;
    let address = format!("{}:{}", config_values.host, config_values.port);
    let mut stream = TcpStream::connect(&address).await?;
    println!("Connected to the server at {}", address);

    Ok(())
}
