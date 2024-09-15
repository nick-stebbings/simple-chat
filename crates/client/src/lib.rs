use std::{error::Error, sync::Arc};
mod cli;
use cli::Args;
use common::{command::Command, connection::Connection};
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
    let connection_handle = task::spawn(async move {
        println!("Connected to the server at {}", address);

        let send_task = async {
            if let Err(e) = connection_clone
                .lock()
                .await
                .send_command(Command::Join(opts.username))
                .await
            {
                println!("Could not register user in server: {}", e);
                return Err::<(), Box<dyn Error + Sync + Send + 'static>>(Box::new(e));
            }
            while let Some(command) = rx.recv().await {
                println!("Sending command: {:?}", command); // Log sending command
                match connection_clone.lock().await.send_command(command).await {
                    Ok(_) => println!("Command sent successfully"), // Log success
                    Err(e) => {
                        println!("Failed to send command: {}", e); // Log error
                        return Err::<(), Box<dyn Error + Sync + Send + 'static>>(Box::new(e));
                    }
                }
            }
            Ok::<(), Box<dyn Error + Sync + Send + 'static>>(())
        };

        let recv_task = async {
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
        };

        tokio::join!(send_task, recv_task)
    });

    let cli_handle = task::spawn(async move {
        loop {
            cli::run_cli(sender_ref.clone()).await;
        }
    });

    match tokio::join!(connection_handle, cli_handle) {
        (Ok(_res1), Ok(_res2)) => Ok(()),
        (Err(e), _) | (_, Err(e)) => Err(Box::new(e)),
    }
}
