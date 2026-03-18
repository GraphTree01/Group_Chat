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