#![allow(dead_code)]
#![allow(unused_variables)]

use crate::command::Command;
use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use tokio::io::{duplex, AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};

pub struct Connection<S> {
    pub framed: Framed<S, LinesCodec>,
}

impl<S: AsyncWrite + AsyncRead + Unpin> Connection<S> {
    pub fn new(stream: S) -> Self {
        let framed = Framed::<S, LinesCodec>::new(stream, LinesCodec::new());
        Connection { framed }
    }
    pub async fn send_command(&mut self, command: Command) -> Result<(), LinesCodecError> {
        self.framed.send(command).await?;
        <Framed<S, LinesCodec> as SinkExt<Command>>::flush(&mut self.framed).await?;
        Ok(())
    }
    pub async fn read_command(&mut self) -> Result<Option<Command>, LinesCodecError> {
        if let Some(result) = self.framed.next().await {
            match result {
                Ok(line) => Ok(Command::parse(&line.to_string())),
                Err(e) => Err(e.into()),
            }
        } else {
            println!("Client closed connection");
            Ok(None)
        }
    }
}
#[tokio::test]
async fn test_read_send_message_command() {
    // Arrange
    let (client, mut server) = tokio::io::duplex(64);
    let mut connection = Connection::new(client);

    // Act
    let command_str = "send Hey!";
    server.write_all(command_str.as_bytes()).await.unwrap();
    server.write_all(b"\n").await.unwrap();
    let command = connection.read_command().await.unwrap();

    // Assert
    assert_eq!(
        command.unwrap().to_string(),
        Command::SendMessage("Hey!".to_string()).to_string()
    );
}
#[tokio::test]
async fn test_read_leave_command() {
    // Arrange
    let (client, mut server) = tokio::io::duplex(64);
    let mut connection = Connection::new(client);

    // Act
    let command_str = "leave";
    server.write_all(command_str.as_bytes()).await.unwrap();
    server.write_all(b"\n").await.unwrap();
    let command = connection.read_command().await.unwrap();

    // Assert
    assert_eq!(command.unwrap().to_string(), Command::Leave.to_string());
}
#[tokio::test]
async fn test_read_join_command() {
    // Arrange
    let (client, mut server) = tokio::io::duplex(64);
    let mut connection = Connection::new(client);

    // Act
    let command_str = "join Davey";
    server.write_all(command_str.as_bytes()).await.unwrap();
    server.write_all(b"\n").await.unwrap();
    let command = connection.read_command().await.unwrap();

    // Assert
    assert_eq!(
        command.unwrap().to_string(),
        Command::Join("Davey".to_string()).to_string()
    );
}

#[tokio::test]
async fn test_send_send_message_command() {
    // Arrange
    let (client, mut server) = duplex(64);
    let mut connection = Connection::new(client);

    // Act
    let command = Command::SendMessage("Hey!".to_string());
    connection.send_command(command).await.unwrap();

    let mut framed = Framed::new(&mut server, LinesCodec::new());
    let line = framed.next().await.unwrap().unwrap();

    // Assert
    assert_eq!(
        Command::parse(&line.to_string()).unwrap().to_string(),
        "Hey!"
    );
}

#[tokio::test]
async fn test_send_and_read_subsequent_messages() {
    // Arrange
    let (client, server) = tokio::io::duplex(64);
    let mut connection_client = Connection::new(client); // Use the client side for sending
    let mut connection_server = Connection::new(server); // Use the server side for reading

    // Act
    let command1 = Command::SendMessage("Hey".to_string());
    connection_client.send_command(command1).await.unwrap();

    let command2 = Command::SendMessage("Jude!".to_string());
    connection_client.send_command(command2).await.unwrap();

    let command3 = Command::SendMessage("Don't let me down".to_string());
    connection_client.send_command(command3).await.unwrap();

    // Read commands
    let read_command1 = connection_server
        .read_command()
        .await
        .expect("No more commands to read")
        .expect("Not a valid command");
    let read_command2 = connection_server
        .read_command()
        .await
        .expect("No more commands to read")
        .expect("Not a valid command");
    let read_command3 = connection_server
        .read_command()
        .await
        .expect("No more commands to read")
        .expect("Not a valid command");

    // Assert
    assert_eq!(read_command1.to_string(), "Hey");
    assert_eq!(read_command2.to_string(), "Jude!");
    assert_eq!(read_command3.to_string(), "Don't let me down");
}
