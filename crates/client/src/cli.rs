use clap::{arg, command, Parser};
use common::command::Command;
use futures_util::stream::StreamExt;
use std::sync::Arc;
use tokio::io::{self as tokio_io};
use tokio::sync::mpsc::Sender;
use tokio_util::codec::{FramedRead, LinesCodec};

pub async fn run_cli(tx: Arc<Sender<Command>>) {
    let stdin = tokio_io::stdin();
    let mut reader = FramedRead::new(stdin, LinesCodec::new());

    loop {
        print!("\n\rEnter command (send <MSG>/leave): ");

        let line = match reader.next().await.transpose() {
            Ok(Some(line)) => line.trim().to_string(),
            Ok(None) => {
                eprintln!("No input received");
                continue;
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        };

        match parse_command(&line) {
            Some(command) => {
                if let Err(e) = tx.send(command).await {
                    eprintln!("Error sending command: {}", e);
                }
            }
            None => println!("\n\rInvalid command, please try again."),
        }
    }
}

fn parse_command(input: &str) -> Option<Command> {
    if input.starts_with("send ") {
        let msg = input.strip_prefix("send ")?.to_string();
        Some(Command::SendMessage(msg))
    } else if input == "leave" {
        Some(Command::Leave)
    } else {
        None
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "anon")]
    pub username: String,
}