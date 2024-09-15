#![allow(dead_code)]
#![allow(unused_variables)]

use common::command::Command;
use common::connection::Connection;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::{mpsc, Mutex};

use crate::user_pool::UserPool;

pub struct User<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub username: String,
    pub msg_sender: mpsc::Sender<String>,
    pub msg_receiver: Receiver<String>,
    pub conn: Connection<S>,
}

type Receiver<S> = Arc<Mutex<mpsc::Receiver<S>>>;

impl<S: AsyncRead + AsyncWrite + Unpin> User<S> {
    pub async fn handle_commands(&mut self, user_pool: Arc<&UserPool<S>>) {
        loop {
            match self.conn.read_command().await.unwrap() {
                Some(Command::SendMessage(message)) => {
                    println!("user handling command.. {}", message.clone());
                    let _send = self.msg_sender.send(format!("send {}", message)).await;
                }
                Some(Command::Leave) => {
                    println!("user handling leave command.. ");
                    let _send = self.msg_sender.send("leave".to_string()).await;
                    break;
                }
                _ => {
                    break;
                }
            }
        }
    }
}
