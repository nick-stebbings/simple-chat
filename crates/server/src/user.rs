#![allow(dead_code)]
#![allow(unused_variables)]

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc;

pub struct User<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub username: String,
    pub msg_sender: mpsc::Sender<String>,
    pub msg_receiver: mpsc::Receiver<String>,
    pub stream: S,
}
