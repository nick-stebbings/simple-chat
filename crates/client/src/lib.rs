use std::{error::Error, sync::Arc};
mod cli;
use cli::Args;
use common::{command::Command, connection::Connection};
use log::debug;
use tokio::sync::Notify;
use tokio::{
    net::TcpStream,
    sync::{mpsc, Mutex},
    task,
};

pub async fn run(address: String) -> Result<(), Box<dyn Error + Sync + Send>> {
    let opts: Args = <Args as clap::Parser>::parse();
    println!("Greetings, {:?}!", opts.username);
    let socket = TcpStream::connect(address.clone()).await;
    let connection = Arc::new(Mutex::new(Connection::<TcpStream>::new(socket?)));

    let (tx, mut rx) = mpsc::channel::<Command>(1024);
    let sender_ref = Arc::new(tx);
    let connection_clone = connection.clone();

    let notify = Arc::new(Notify::new());
    let notify_for_join = notify.clone();
    let notify_for_cli = notify.clone();

    let connection_handle = task::spawn(async move {
        let tt_connection = connection.clone();
        tokio::spawn(async move {
            let mut connection_lock = tt_connection.lock().await;
            connection_lock
                .send_command(Command::Join(opts.username))
                .await
                .unwrap();
            println!("Joined server at: {:?}", address.clone());
            notify_for_join.notify_one();
            notify_for_join.notify_one();
        });

        notify.notified().await;

        while let Some(command) = rx.recv().await {
            let tt_connection = connection.clone();
            tokio::spawn(async move {
                let mut connection_lock = tt_connection.lock().await;
                debug!("Sent command : {:?}", command.to_string().clone());
                connection_lock.send_command(command).await.unwrap();
            });
        }

        tokio::spawn(async move {
            loop {
                match connection_clone.lock().await.read_command().await {
                    Ok(Some(command)) => {
                        match command {
                            Command::UsernameTaken => println!("That username has been taken, please restart the client with a different one!"),
                            Command::SendMessage(message) => println!("message: {}", message),
                            _ => ()
                        }
                    }
                    Ok(None) => {
                        println!("Connection closed by server");
                        break;
                    }
                    Err(e) => {
                        println!("Failed to read command from server: {}", e);
                        return Err::<(), Box<dyn Error + Sync + Send + 'static>>(Box::new(e));
                    }
                }
            }
            Ok::<(), Box<dyn Error + Sync + Send + 'static>>(())
        });
    });

    let cli_handle = task::spawn(async move {
        notify_for_cli.notified().await;
        loop {
            cli::run_cli(sender_ref.clone()).await;
        }
    });

    match tokio::join!(connection_handle, cli_handle) {
        (Ok(_res1), Ok(_res2)) => Ok(()),
        (Err(e), _) | (_, Err(e)) => Err(Box::new(e)),
    }
}
