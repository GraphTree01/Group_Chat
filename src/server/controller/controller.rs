use crate::server::model::server::{IdentifyResult, Server};
use crate::server::view::glossary;
use crate::shared::protocol::Message;
use crate::shared::verifier::verify;
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::IpAddr;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

pub fn run() {
    read();
}

fn read() {
    let args: Vec<String> = env::args().collect();

    match verify(args) {
        Ok(parsed) => {
            let ip: IpAddr = parsed.ip.unwrap_or("0.0.0.0".parse().unwrap());
            let port: u16 = parsed.port.unwrap_or(1234);
            let server = Arc::new(Server::new());
            let _ = start_server(ip, port, server);
        }
        Err(err) => {
            eprintln!("{err}");
        }
    }
}

fn start_server(ip: IpAddr, port: u16, server: Arc<Server>) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{ip}:{port}"))?;

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let server = Arc::clone(&server);
            thread::spawn(move || {
                let _ = handle_client(stream, server);
            });
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, server: Arc<Server>) -> std::io::Result<()> {
    let reader_stream = stream.try_clone()?;
    let mut reader = BufReader::new(reader_stream);
    let mut line = String::new();
    let mut identified_username: Option<String> = None;

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line)?;

        if bytes_read == 0 {
            break;
        }

        let payload = line.trim();
        if payload.is_empty() {
            continue;
        }

        match serde_json::from_str::<Message>(payload) {
            Ok(Message::Identify { username }) => {
                glossary::identify_received(&username);

                match server.identify(username.clone())? {
                    IdentifyResult::Success { response, new_user } => {
                        send_message(&mut stream, &response)?;
                        glossary::identify_success_response(&username);

                        // Broadcast is not implemented yet; this line keeps terminal output aligned with protocol flow.
                        let _ = new_user;
                        glossary::new_user(&username);
                        identified_username = Some(username);
                    }
                    IdentifyResult::UserAlreadyExists { response } => {
                        send_message(&mut stream, &response)?;
                        glossary::identify_user_already_exists_response(&username);
                    }
                }
            }
            Ok(_) => {}
            Err(_) => {}
        }
    }

    if let Some(username) = identified_username {
        server.disconnect(&username)?;
    }

    Ok(())
}

fn send_message(stream: &mut TcpStream, message: &Message) -> std::io::Result<()> {
    let encoded = serde_json::to_string(message)
        .map_err(|err| std::io::Error::other(format!("cannot encode message: {err}")))?;
    stream.write_all(encoded.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;
    Ok(())
}
