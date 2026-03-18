use std::collections::HashMap;
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

use crate::shared::user::User;

pub struct Server {

    ip: String,
    port: String,
    users: HashMap<String, User>, 
}

impl Server {
    pub fn new(ip: String, port: String) -> Self {
        Self {
            ip,
            port,
            users: HashMap::new(),
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn start(&mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.address())?;
        println!("Servidor en {}", self.address());

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_client(stream)?,
                Err(err) => eprint!("Error aceptando conexión: {err}"),
            }
        }

        Ok(())
    }

    fn handle_client(&mut self, stream: TcpStream) -> std::io::Result<()> {
        Ok(())
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.username.clone(), user);
    }
}

