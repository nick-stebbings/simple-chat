use crate::user::User;
use std::{collections::HashMap, sync::RwLock};
use tokio::io::{AsyncRead, AsyncWrite};

/**
 * Manages the Users. Generic over a stream so that TcpStream can be mocked in unit tests.
 */
pub struct UserPool<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    users: RwLock<HashMap<String, User<S>>>,
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

    pub async fn add_user(&self, user: User<S>) {
        if let Ok(mut hashmap) = self.users.write() {
            let username = user.username.clone();
            hashmap.insert(username, user);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::{
        io::{duplex, DuplexStream},
        sync::mpsc,
    };

    #[tokio::test]
    async fn test_add_user() {
        // Arrange
        // Create a mock TcpStream like object
        let (stream1, _stream2) = duplex(64);

        let user_pool = UserPool::<DuplexStream>::new();
        let (tx, rx) = mpsc::channel(50);
        let user = User {
            username: "anon".to_string(),
            msg_sender: tx,
            msg_receiver: rx,
            stream: stream1,
        };

        // Act
        user_pool.add_user(user).await;
        let users = user_pool.users.read().unwrap();

        // Assert
        assert!(users.contains_key("anon"));
    }
}
