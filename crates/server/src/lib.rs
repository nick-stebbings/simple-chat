mod user;
mod user_pool;

use tokio::net::TcpListener;

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut _reader, mut _writer) = socket.split();
            let mut _buf = vec![0; 1024];
        });
    }
}
