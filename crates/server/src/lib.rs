mod user;
mod user_pool;

use std::sync::Arc;

use common::{
    command::{parse_command, Command},
    connection::Connection,
};
use log::debug;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex},
};
use user::User;
use user_pool::UserPool;

pub async fn run(address: String) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(&address).await?;
    println!("Server running on {}", address);
    let user_pool = Arc::new(UserPool::<TcpStream>::new());

    loop {
        let (socket, _) = listener.accept().await?;
        let user_pool: Arc<UserPool<TcpStream>> = user_pool.clone();

        tokio::spawn(async move {
            let mut connection = Connection::new(socket);

            match connection.read_command().await {
                Ok(Some(Command::Join(username))) => {
                    // Channels for communication
                    let (tx_user_to_pool, rx_pool_from_user) = mpsc::channel(200);
                    let (_, user_from_pool) = mpsc::channel::<String>(1024);

                    let connected_user = User {
                        username: username.clone(),
                        msg_sender: tx_user_to_pool.clone(),
                        msg_receiver: Arc::new(Mutex::new(user_from_pool)),
                        conn: connection,
                    };
                    let user = Arc::new(Mutex::new(connected_user));
                    user_pool.add_user(user.clone()).await;

                    let user_cloned = user.clone();
                    let user_pool_cloned = user_pool.clone();
                    // Spawn a task to handle incoming commands from this user's connection
                    tokio::spawn(async move {
                        println!("Server processing commands:");
                        let mut local_rx = rx_pool_from_user; // Take ownership of the receiver
                        while let Some(message) = local_rx.recv().await {
                            println!("Server processing command: {:?}", message);
                            // user_pool_cloned
                            //     .process_command(parse_command(message.as_str()), user.clone())
                            //     .await;
                        }
                    });
                    // Spawn a separate task for this user to handle their commands
                    let user_pool_cloned_for_comm = user_pool.clone();
                    let user = user_cloned.clone();
                    tokio::spawn(async move {
                        user.lock()
                            .await
                            .handle_commands(user_pool_cloned_for_comm)
                            .await;
                    });
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error reading initial command: {}", e);
                }
            }
        });
    }
}
