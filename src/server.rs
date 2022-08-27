use std::{io::Read, net::TcpListener};

use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", self.addr);

        loop {
            match listener.accept() {
                Ok((stream, _)) => Self::process_stream(stream),
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }

    fn process_stream(mut stream: std::net::TcpStream) {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Ok(_) => match Request::try_from(&buffer[..]) {
                Ok(request) => {
                    dbg!(request);
                }
                Err(_) => {}
            },

            Err(e) => println!("Error while processing stream: {}", e),
        }
    }
}
