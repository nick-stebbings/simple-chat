#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::{mpsc, Mutex};

pub struct User<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub username: String,
    pub msg_sender: mpsc::Sender<String>,
    pub msg_receiver: Receiver<String>,
    pub stream: S,
}

type Receiver<S> = Arc<Mutex<mpsc::Receiver<S>>>;
