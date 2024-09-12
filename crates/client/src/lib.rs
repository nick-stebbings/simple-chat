#![allow(dead_code)]
#![allow(unused_variables)]
use log::debug;
use std::{error::Error, sync::Arc};
mod cli;
use cli::Args;
use common::{command::Command, connection::Connection};
use tokio::{sync::mpsc, task};

pub async fn run(address: String) -> Result<(), Box<dyn Error + Sync + Send>> {
    let opts: Args = <Args as clap::Parser>::parse();
    let (tx, mut rx) = mpsc::channel::<Command>(32);
    let sender_ref = Arc::new(tx);

    let connection_handle = task::spawn(async move {
        match Connection::new(&address).await {
            Ok(connection) => {
                debug!("Connected to the server at {}", address);

                while let Some(command) = rx.recv().await {
                    println!("command received from cli {:?}", command);
                }

                Ok::<(), Box<dyn Error + Sync + Send + 'static>>(())
            }
            Err(e) => {
                debug!("Failed to connect to the server at {}: {}", address, e);
                Err::<(), Box<dyn Error + Sync + Send + 'static>>(e)
            }
        }
    });

    let cli_handle = task::spawn(async move {
        loop {
            cli::run_cli(sender_ref.clone()).await;
        }
    });

    match tokio::join!(connection_handle, cli_handle) {
        (Ok(res1), Ok(res2)) => res1,
        (Err(e), _) | (_, Err(e)) => Err(Box::new(e)),
    }
}
