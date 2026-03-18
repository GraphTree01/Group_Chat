use crate::server::model::server::Server;
use crate::shared::verifier::verify;
use std::env;
use std::net::IpAddr;

pub fn run() {
    read();
}

fn read() {
    let args: Vec<String> = env::args().collect();

    match verify(args) {
        Ok(parsed) => {
            let ip: IpAddr = parsed.ip.unwrap_or("0.0.0.0".parse().unwrap());
            let port: u16 = parsed.port.unwrap_or(1234);
            let mut server = Server::new(ip.to_string(), port.to_string());
            let _ = server.start();
        }
        Err(err) => {
            eprintln!("{err}");
        }
    }
}
