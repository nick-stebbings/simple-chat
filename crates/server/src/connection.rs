#![allow(dead_code)]
#![allow(unused_variables)]
use crate::command::Command;
use bytes::BytesMut;
use futures_util::*;
use tokio::io::{self, BufWriter};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, LinesCodec};
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn read_command(&mut self) -> io::Result<Option<Command>> {
        let mut framed = FramedRead::new(&mut self.stream, LinesCodec::new());

        while let Ok(Some(line)) = framed.try_next().await {
            if let Some(command) = Command::parse(&line) {
                return Ok(Some(command));
            }
        }
        Ok(None)
    }
}
