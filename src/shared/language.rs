use std::error::Error;
use std::fmt;

use crate::shared::protocol::Message;
use crate::shared::verifier::verify_username;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientCommand {
    Identify { username: String },
}

impl ClientCommand {
    pub fn into_message(self) -> Message {
        match self {
            Self::Identify { username } => Message::identify(username),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseCommandError {
    EmptyInput,
    UnknownCommand(String),
    MissingArgument { command: &'static str },
    TooManyArguments { command: &'static str },
    InvalidUsername(String),
}

impl fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "no command provided"),
            Self::UnknownCommand(cmd) => write!(f, "unknown command: {cmd}"),
            Self::MissingArgument { command } => {
                write!(f, "missing required argument for command: {command}")
            }
            Self::TooManyArguments { command } => {
                write!(f, "too many arguments for command: {command}")
            }
            Self::InvalidUsername(reason) => write!(f, "invalid username: {reason}"),
        }
    }
}

impl Error for ParseCommandError {}

pub fn parse_user_command(input: &str) -> Result<ClientCommand, ParseCommandError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ParseCommandError::EmptyInput);
    }

    let mut parts = trimmed.split_whitespace();
    let raw_command = parts.next().ok_or(ParseCommandError::EmptyInput)?;
    let command = raw_command.to_ascii_lowercase();

    match command.as_str() {
        "identify" => parse_identify(parts),
        _ => Err(ParseCommandError::UnknownCommand(raw_command.to_string())),
    }
}

fn parse_identify<'a>(
    mut args: impl Iterator<Item = &'a str>,
) -> Result<ClientCommand, ParseCommandError> {
    let username = args.next().ok_or(ParseCommandError::MissingArgument {
        command: "identify",
    })?;

    if args.next().is_some() {
        return Err(ParseCommandError::TooManyArguments {
            command: "identify",
        });
    }

    verify_username(username)
        .map_err(|reason| ParseCommandError::InvalidUsername(reason.to_string()))?;

    Ok(ClientCommand::Identify {
        username: username.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_identify_command() {
        let parsed = parse_user_command("identify Kimberly").expect("parse identify command");

        assert_eq!(
            parsed,
            ClientCommand::Identify {
                username: "Kimberly".to_string(),
            }
        );
    }

    #[test]
    fn parses_identify_with_mixed_case_command() {
        let parsed = parse_user_command("IdEnTiFy Kimberly").expect("parse identify command");

        assert_eq!(
            parsed,
            ClientCommand::Identify {
                username: "Kimberly".to_string(),
            }
        );
    }

    #[test]
    fn rejects_unknown_command() {
        let err = parse_user_command("login Kimberly").expect_err("unknown command error");

        assert_eq!(err, ParseCommandError::UnknownCommand("login".to_string()));
    }

    #[test]
    fn rejects_identify_without_username() {
        let err = parse_user_command("identify").expect_err("missing argument error");

        assert_eq!(
            err,
            ParseCommandError::MissingArgument {
                command: "identify"
            }
        );
    }

    #[test]
    fn rejects_identify_with_invalid_username() {
        let err = parse_user_command("identify ki!").expect_err("invalid username error");

        assert_eq!(
            err,
            ParseCommandError::InvalidUsername(
                "must contain only ASCII letters, numbers, or '_'".to_string()
            )
        );
    }

    #[test]
    fn converts_command_to_protocol_message() {
        let cmd = ClientCommand::Identify {
            username: "Kimberly".to_string(),
        };

        let message = cmd.into_message();

        match message {
            Message::Identify { username } => assert_eq!(username, "Kimberly"),
            _ => panic!("unexpected message variant"),
        }
    }
}
