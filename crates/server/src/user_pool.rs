use crate::user::User;
use common::command::Command;
use log::debug;
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::{Mutex, RwLock},
};

/**
 * Manages the Users. Generic over a stream so that TcpStream can be mocked in unit tests.
 */
pub struct UserPool<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    users: RwLock<HashMap<String, Arc<Mutex<User<S>>>>>,
}

impl<S> UserPool<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new() -> Self {
        UserPool {
            users: RwLock::new(HashMap::new()),
        }
    }

    /**
     * Adds a unique user to the user pool.
     */
    pub async fn add_user(&self, user: Arc<Mutex<User<S>>>) {
        let mut hashmap = self.users.write().await;

        let username = user.lock().await.username.clone();

        match hashmap.entry(username) {
            Entry::Occupied(_) => {
                let _ = user
                    .lock()
                    .await
                    .msg_sender
                    .send("username_taken".to_string())
                    .await;
            }
            Entry::Vacant(entry) => {
                entry.insert(user);
            }
        }
    }

    /**
     * Removes a user from the user pool
     */
    pub async fn remove_user_with_username(&self, username: String) {
        let mut hashmap = self.users.write().await;
        if let Entry::Occupied(entry) = hashmap.entry(username) {
            entry.remove();
        }
    }

    /**
     * Broadcasts a message to all other users
     */
    pub async fn broadcast(&self, sender_username: String, message: &str) {
        let users = self.users.read().await;
        for (username, user) in users.iter() {
            if username.as_str() != sender_username {
                debug!("SENDING");
                let _ = user.lock().await.msg_sender.send(message.to_string()).await;
            }
        }
    }
    /**
     * Broadcasts a message to the client telling them to choose another username
     */
    pub async fn alert_duplicate_username(&self, user: Arc<Mutex<User<S>>>) {
        let _send = user
            .lock()
            .await
            .conn
            .send_command(Command::UsernameTaken)
            .await;
    }

    /**
     * Processes a command from a user.
     */
    pub async fn process_command(&self, command: Option<Command>, user: Arc<Mutex<User<S>>>) {
        debug!("Handling command");
        match command {
            Some(Command::SendMessage(message)) => {
                self.broadcast(user.lock().await.username.clone(), &message)
                    .await;
            }
            Some(Command::Leave) => {
                self.remove_user_with_username(user.lock().await.username.clone())
                    .await;
            }
            Some(Command::UsernameTaken) => {
                self.alert_duplicate_username(user.clone()).await;
            }
            _ => {
                println!("Not a valid command :(")
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use common::connection::Connection;
    use tokio::{
        io::{duplex, DuplexStream},
        sync::{mpsc, Mutex},
    };

    #[tokio::test]
    async fn test_add_user() {
        // Arrange
        // Create a mock TcpStream like object
        let (stream1, _stream2) = duplex(64);

        let user_pool = UserPool::<DuplexStream>::new();
        let (tx, rx) = mpsc::channel(5);
        let user = User {
            username: "anon".to_string(),
            msg_sender: tx,
            msg_receiver: Arc::new(Mutex::new(rx)),
            conn: Connection::new(stream1),
        };

        // Act
        user_pool.add_user(Arc::new(Mutex::new(user))).await;
        let users = user_pool.users.read().await;

        // Assert
        assert!(users.contains_key("anon"));
    }
    #[tokio::test]
    async fn test_add_distinct_user() {
        // Arrange
        let (stream1, _) = duplex(64);
        let (stream2, __) = duplex(64);

        let user_pool = UserPool::<DuplexStream>::new();
        let (tx1, rx1) = mpsc::channel(5);
        let (tx2, rx2) = mpsc::channel(5);
        let user1 = User {
            username: "anon".to_string(),
            msg_sender: tx1,
            msg_receiver: Arc::new(Mutex::new(rx1)),
            conn: Connection::new(stream1),
        };
        let user2 = User {
            username: "anon2".to_string(),
            msg_sender: tx2,
            msg_receiver: Arc::new(Mutex::new(rx2)),
            conn: Connection::new(stream2),
        };

        // Act
        user_pool.add_user(Arc::new(Mutex::new(user1))).await;
        user_pool.add_user(Arc::new(Mutex::new(user2))).await;
        let users = user_pool.users.read().await;

        // Assert
        assert!(users.contains_key("anon"));
        assert!(users.contains_key("anon2"));
    }
    #[tokio::test]
    async fn test_add_same_username() {
        // Arrange
        let (stream1, _) = duplex(64);
        let (stream2, __) = duplex(64);

        let user_pool = UserPool::<DuplexStream>::new();
        let (tx1, rx1) = mpsc::channel(5);
        let (tx2, rx2) = mpsc::channel(5);
        let user1 = User {
            username: "anon".to_string(),
            msg_sender: tx1,
            msg_receiver: Arc::new(Mutex::new(rx1)),
            conn: Connection::new(stream1),
        };
        let user2 = User {
            username: "anon".to_string(),
            msg_sender: tx2,
            msg_receiver: Arc::new(Mutex::new(rx2)),
            conn: Connection::new(stream2),
        };

        // Act
        user_pool.add_user(Arc::new(Mutex::new(user1))).await;
        user_pool.add_user(Arc::new(Mutex::new(user2))).await;
        let users = user_pool.users.read().await;

        // Assert
        assert!(users.contains_key("anon"));
        assert_eq!(users.len(), 1);
    }
    #[tokio::test]
    async fn test_drop_user() {
        // Arrange
        let (stream1, _) = duplex(64);

        let user_pool = UserPool::<DuplexStream>::new();
        let (tx1, rx1) = mpsc::channel(5);
        let user1 = User {
            username: "anon".to_string(),
            msg_sender: tx1,
            msg_receiver: Arc::new(Mutex::new(rx1)),
            conn: Connection::new(stream1),
        };

        // Act
        let username = user1.username.clone();
        user_pool.add_user(Arc::new(Mutex::new(user1))).await;
        user_pool.remove_user_with_username(username).await;
        let users = user_pool.users.read().await;

        // Assert
        assert_eq!(users.len(), 0);
    }
    #[tokio::test]
    async fn test_user_sends_message() {
        // Arrange
        let (stream1, _) = duplex(64);
        let (stream2, __) = duplex(64);

        let user_pool = UserPool::<DuplexStream>::new();
        let (tx1, rx1) = mpsc::channel(5);
        let (tx2, rx2) = mpsc::channel(5);
        let rx2_by_ref = Arc::new(Mutex::new(rx2));

        let user1 = User {
            username: "anon".to_string(),
            msg_sender: tx1.clone(),
            msg_receiver: Arc::new(Mutex::new(rx1)),
            conn: Connection::new(stream1),
        };
        let user2 = User {
            username: "anon2".to_string(),
            msg_sender: tx2,
            msg_receiver: rx2_by_ref.clone(),
            conn: Connection::new(stream2),
        };

        // Act
        let user1_name = user1.username.clone();
        user_pool.add_user(Arc::new(Mutex::new(user1))).await;
        user_pool.add_user(Arc::new(Mutex::new(user2))).await;

        user_pool.broadcast(user1_name, "Hello world!").await;

        // Assert

        let rx2_ref_temp = rx2_by_ref.clone();
        let mut rx2_ref = rx2_ref_temp.lock().await;
        assert_eq!(rx2_ref.recv().await, Some("Hello world!".to_string()));
    }
    #[tokio::test]
    async fn test_user_does_not_receive_own_sent_message() {
        // Arrange
        let (stream1, _) = duplex(64);
        let (stream2, __) = duplex(64);

        let user_pool = UserPool::<DuplexStream>::new();
        let (tx1, rx1) = mpsc::channel(5);
        let (tx2, rx2) = mpsc::channel(5);
        let rx1_by_ref = Arc::new(Mutex::new(rx1));
        let rx2_by_ref = Arc::new(Mutex::new(rx2));

        let user1 = User {
            username: "anon".to_string(),
            msg_sender: tx1.clone(),
            msg_receiver: rx1_by_ref.clone(),
            conn: Connection::new(stream1),
        };
        let user2 = User {
            username: "anon2".to_string(),
            msg_sender: tx2,
            msg_receiver: rx2_by_ref.clone(),
            conn: Connection::new(stream2),
        };

        // Act
        let user1_name = user1.username.clone();
        user_pool.add_user(Arc::new(Mutex::new(user1))).await;
        user_pool.add_user(Arc::new(Mutex::new(user2))).await;

        user_pool.broadcast(user1_name, "Hello world!").await;

        // Assert

        let rx1_ref_temp = rx1_by_ref.clone();
        let mut rx1_ref = rx1_ref_temp.lock().await;
        // We are only testing that a None value is taken from the receiver channel, so can panic otherwise
        match rx1_ref.try_recv().ok() {
            Some(_) => panic!("User 1 should not receive their own message"),
            None => assert!(true),
        }
    }
}
