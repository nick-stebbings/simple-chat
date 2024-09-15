# Chat Server & Client

## Problem: 
### Basic requirements
- one chat room.
- users may freely join or leave (no auth/authz)
- user may send messages to the room
- messages will be sent to all connected users minus the sender.

- high throughput: all code should be non-blocking for maximum concurrency.

### Extra requirements
- include both unit and integration tests where necessary.
- format using the standard formatting tool.
- code must compile without clippy errors.

### Bonus
- git commit hook for formatting, compilation, linting
- GH action send a message from client to server without failure, run on push/commit to main

## User Stories
### Client
  User Story 1: Connect to the Server
    As a client user,
    I want to specify the host, port, and username via environment variables or command line arguments, so that I can connect to the chat server with my unique identifier.

  User Story 2: Automatic Connection
    As a client user,
    I want the client to automatically connect to the chat server upon initialization using the specified host and port, so that I can start chatting without manual connection steps.

  User Story 3: Interactive Command Prompt
    As a client user,
    I want the client to display an interactive command prompt, so that I can easily send messages and manage my connection.

  User Story 4: Send Messages
    As a client user,
    I want to use the command `send <MSG>` to send a message to the server, so that my message is broadcasted to all other connected users.

  User Story 5: Receive Messages
    As a client user,
    I want to see any messages sent from the server displayed in my command prompt, so that I can stay updated with the chat room conversation.

  User Story 6: Leave the Chat
    As a client user,
    I want to use the command leave to disconnect from the server and exit the CLI, so that I can gracefully leave the chat room when I am done.

## Server Requirements:
- Any other user who is currently connected should get the message sent to them.
- The user who sent the message should not get the message.
- When a user sends a leave message, or disconnects their client, the server should no longer send messages to them, and do any internal bookkeeping to clean up.
- Enforce name uniqueness (when a use disconnects they will be removed from the UserPool)

### Performance
The server should be able to support many users without a large delay
  -- use tokio to spawn a thread for each user
  -- use Rwlock as users will be written far less than read.
  -- Use Arc to pass multiple shared references to a mutable UserPool
The server should be able to support many users with a small memory footprint
 -- use lazy async in tokio, preserving memory

## Crates needed
- tokio
 -- spawn
 -- messaging
 -- tcp
- clap
 - args: [username, msg]
- serde
- dotenv

## Data structures
### User
struct User
  username - String
  msg_sender - Sender
  msg_receiver - Receiver
  strem - TCPStream
#### Responsibilities:
- Sends and receives messages to other users

### User Manager
struct UserPool
  users: RWLock<HashMap<String, User>> (username -> user)
#### Responsibilities:
- Keeps track of server users
- Guards for name uniqueness
- Must be threadsafe, non-blocking