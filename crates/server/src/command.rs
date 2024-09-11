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
