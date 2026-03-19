use clap::Parser;
use std::net::IpAddr;

#[derive(Debug, Parser)]
#[command(name = "server")]
pub struct ServerArgs {
    #[arg(long, value_parser = clap::value_parser!(IpAddr))]
    pub ip: Option<IpAddr>,

    #[arg(long, value_parser = clap::value_parser!(u16).range(1..=65535))]
    pub port: Option<u16>,
}

pub fn verify(args: Vec<String>) -> Result<ServerArgs, clap::Error> {
    ServerArgs::try_parse_from(args)
}

pub fn verify_username(username: &str) -> Result<(), &'static str> {
    if username.is_empty() {
        return Err("username cannot be empty");
    }

    if username.len() < 3 {
        return Err("must be at least 3 characters long");
    }

    if username.len() > 8 {
        return Err("must be at most 8 characters long");
    }

    if !username
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    {
        return Err("must contain only ASCII letters, numbers, or '_'");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_username() {
        let result = verify_username("kim_2026");
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_short_username() {
        let result = verify_username("ab");
        assert_eq!(result, Err("must be at least 3 characters long"));
    }

    #[test]
    fn rejects_username_with_invalid_chars() {
        let result = verify_username("kim!");
        assert_eq!(
            result,
            Err("must contain only ASCII letters, numbers, or '_'")
        );
    }
}
