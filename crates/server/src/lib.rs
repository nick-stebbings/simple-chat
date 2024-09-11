mod user;
mod user_pool;

use tokio::net::TcpListener;

pub async fn run(address: String) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(&address).await?;
    println!("Server running on {}", address);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut _reader, mut _writer) = socket.split();
            let mut _buf = vec![0; 1024];
        });
    }
}
