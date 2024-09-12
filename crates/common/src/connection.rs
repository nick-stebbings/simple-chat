#![allow(dead_code)]
#![allow(unused_variables)]
use bytes::BytesMut;
use futures_util::*;
use tokio::io::{self, BufWriter};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, LinesCodec};

use crate::command::Command;
pub struct Connection {
    stream: Option<BufWriter<TcpStream>>,
    buffer: BytesMut,
}

impl Connection {
    pub async fn new(
        address: &str,
    ) -> Result<Connection, Box<dyn std::error::Error + Send + Sync>> {
        match TcpStream::connect(address).await {
            Ok(socket) => Ok(Connection {
                stream: Some(BufWriter::new(socket)),
                buffer: BytesMut::with_capacity(4 * 1024),
            }),
            Err(e) => {
                eprintln!("Failed to connect to the server at {}: {}", address, e);
                Err(Box::new(e))
            }
        }
    }

    pub async fn read_command(&mut self) -> io::Result<Option<Command>> {
        if let Some(ref mut stream) = self.stream {
            let mut framed = FramedRead::new(stream, LinesCodec::new());

            while let Ok(Some(line)) = framed.try_next().await {
                if let Some(command) = Command::parse(&line) {
                    return Ok(Some(command));
                }
            }
        }
        Ok(None)
    }
}
