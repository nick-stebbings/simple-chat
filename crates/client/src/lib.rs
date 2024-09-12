use log::debug;
use std::{error::Error, sync::Arc};
mod cli;
use cli::Args;
use common::{command::Command, connection::Connection};
use tokio::{net::TcpStream, sync::mpsc, task};

pub async fn run(address: String) -> Result<(), Box<dyn Error + Sync + Send>> {
    let opts: Args = <Args as clap::Parser>::parse();
    println!("Greetings, {:?}!", opts.username);
    let socket = TcpStream::connect(address.clone()).await;

    let (tx, mut rx) = mpsc::channel::<Command>(32);
    let sender_ref = Arc::new(tx);
    let connection_handle = task::spawn(async move {
        match socket {
            Ok(socket) => {
                let mut connection = Connection::<TcpStream>::new(socket);
                debug!("Connected to the server at {}", address);
                match connection.send_command(Command::Join(opts.username)).await {
                    Ok(_join) => {
                        while let Some(command) = rx.recv().await {
                            let _sent = connection.send_command(command);
                        }

                        Ok::<(), Box<dyn Error + Sync + Send + 'static>>(())
                    }
                    Err(e) => {
                        println!("Could not join server");
                        Err::<(), Box<dyn Error + Sync + Send + 'static>>(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect to the server at {}: {}", address, e);
                Err::<(), Box<dyn Error + Sync + Send + 'static>>(Box::new(e))
            }
        }
    });

    let cli_handle = task::spawn(async move {
        loop {
            cli::run_cli(sender_ref.clone()).await;
        }
    });

    match tokio::join!(connection_handle, cli_handle) {
        (Ok(res1), Ok(_res2)) => res1,
        (Err(e), _) | (_, Err(e)) => Err(Box::new(e)),
    }
}
