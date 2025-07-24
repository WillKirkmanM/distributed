use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Get(String),
    Set(String, String),
    Del(String),
    Begin,
    Commit,
    Rollback,
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Unknown command")]
    Unknown,
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
}

pub fn parse(input: &str) -> Result<Command, CommandError> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    match parts.as_slice() {
        ["GET", key] => Ok(Command::Get(key.to_string())),
        ["SET", key, value] => Ok(Command::Set(key.to_string(), value.to_string())),
        ["DEL", key] => Ok(Command::Del(key.to_string())),
        ["BEGIN"] => Ok(Command::Begin),
        ["COMMIT"] => Ok(Command::Commit),
        ["ROLLBACK"] => Ok(Command::Rollback),
        [] => Err(CommandError::InvalidArguments("empty command".to_string())),
        _ => Err(CommandError::Unknown),
    }
}
