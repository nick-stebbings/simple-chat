use tokio::net::TcpStream;

pub async fn run(address: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut _stream = TcpStream::connect(&address).await?;
    println!("Connected to the server at {}", address);

    Ok(())
}
