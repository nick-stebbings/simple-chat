mod user;
mod user_pool;

use std::sync::Arc;

use common::{command::Command, connection::Connection};
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
        let mut connection = Connection::new(socket);

        if let Some(Command::Join(username)) = connection.read_command().await.unwrap() {
            tokio::spawn(async move {
                // Make a channel from user <-> users
                let (_, rx1) = mpsc::channel(200);
                // Make a channel from user -> user pool
                let (tx, rx2) = mpsc::channel::<String>(1024);
                let user: Arc<Mutex<User<tokio::net::TcpStream>>> = Arc::new(Mutex::new(User {
                    username: username.clone(),
                    msg_sender: tx,
                    msg_receiver: Arc::new(Mutex::new(rx1)),
                    conn: connection,
                }));

                user_pool.add_user(user.clone()).await;
                // Tell the pool to wait for commands from this user
                user_pool.watch_for_next_command(rx2, user.clone()).await;
                // Tell the user to listen on its socket for commands
                user.lock()
                    .await
                    .handle_commands(Arc::new(&user_pool))
                    .await;
            });
        } else {
        }
    }
}
