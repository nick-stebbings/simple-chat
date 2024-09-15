#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Join(String),
    Leave,
    SendMessage(String),
    UsernameTaken,
}

impl Command {
    pub fn parse(line: &str) -> Option<Command> {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        match parts[0] {
            "join" => parts
                .get(1)
                .map(|&username| Command::Join(username.to_string())),
            "leave" => Some(Command::Leave),
            "username_taken" => Some(Command::UsernameTaken),
            "send" => parts
                .get(1)
                .map(|&msg| Command::SendMessage(msg.to_string())),
            _ => None,
        }
    }
}
impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        match self {
            Command::SendMessage(msg) => Box::leak(format!("send {}", msg).into_boxed_str()),
            Command::Join(username) => Box::leak(format!("join {}", username).into_boxed_str()),
            Command::Leave => "leave",
            Command::UsernameTaken => "username_taken",
        }
    }
}
impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::SendMessage(msg) => write!(f, "{}", msg),
            Command::Join(username) => write!(f, "join {}", username),
            Command::Leave => write!(f, "leave"),
            Command::UsernameTaken => write!(f, "username_taken"),
        }
    }
}

pub fn parse_command(input: &str) -> Option<Command> {
    if input.starts_with("send ") {
        let msg = input.strip_prefix("send ")?.to_string();
        Some(Command::SendMessage(msg))
    } else if input == "leave" {
        Some(Command::Leave)
    } else if input == "username_taken" {
        Some(Command::UsernameTaken)
    } else {
        None
    }
}
