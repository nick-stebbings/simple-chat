# Simple Asynchronous Chat Server

This project is a simple asynchronous chat server and CLI client built in Rust. It was developed as a Rust job assignment to demonstrate proficiency in async Rust.

I used Tokio channels, Arc smart pointers for thread-safe sharing of state, and utilise the actor pattern to prevent deadlocks.

## Features

- **Single Room Chat**: Users can join, leave, and send messages in a single chat room.
- **High Throughput**: Designed for maximum concurrency with non-blocking code.
- **Unique Usernames**: Ensures each user has a unique identifier.
- **Efficient Resource Usage**: Supports many users with minimal delay and memory footprint.

## Technologies Used

- **Async Rust**: Async runtime for polling the top-level Future.
- **Smart Pointer & Thread-safe data structure (Arc<Mutex>)**: To manage shared state safely between tasks.
- **Tokio Channels**: For message passing between tasks.

## Server Specifications

- [ ] Manage users and their messages.
- [ ] Process join, leave, and message commands.
- [ ] Broadcast messages to all users except the sender.
- [ ] Clean up resources when users leave or disconnect.
- [ ] Support many users efficiently.

## Client Specifications

- [ ] Async CLI program for user interaction.
- [ ] Connects to server using host and port from environment variables or command line arguments.
- [ ] Interactive command prompt with:
  - [ ] `send <MSG>`: Send a message to the server.
  - [ ] `leave`: Disconnect from the server and exit.

## Additional Requirements

- [ ] Include unit and integration tests.
- [ ] Format code using the standard tool.
- [ ] Ensure code compiles without clippy errors.

## Bonus Features

- [ ] Pre-commit hook for code formatting and error checking.
- [ ] GitHub Action to test server-client interaction on code push.

## Original Submission Instructions:

1. Fork this repository to your GitHub account.
2. Submit a pull request to our repository.
3. Include a demo video at the top of the pull request.
4. Submit the pull request link for review.

### Prerequisites

- Rust installed on your machine.

### Running the Server
`cargo run --bin server`

### Running the Client
`cargo run --bin client`  

### Running Tests
`cargo test`  