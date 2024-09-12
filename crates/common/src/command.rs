#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Command {
    Join(String),
    Leave,
    SendMessage(String),
}

impl Command {
    pub fn parse(line: &str) -> Option<Command> {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        match parts[0] {
            "join" => parts
                .get(1)
                .map(|&username| Command::Join(username.to_string())),
            "leave" => Some(Command::Leave),
            "send" => parts
                .get(1)
                .map(|&msg| Command::SendMessage(msg.to_string())),
            _ => None,
        }
    }
}
impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::SendMessage(msg) => write!(f, "{}", msg),
            Command::Join(username) => write!(f, "join {}", username),
            Command::Leave => write!(f, "leave"),
        }
    }
}
