use std::{
    io::{Read, Write},
    net::TcpListener,
};

use crate::http::{
    response::{self, Response},
    Request, StatusCode,
};

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
            Ok(_) => {
                let response = match Request::try_from(&buffer[..]) {
                    Ok(request) => {
                        dbg!(request);
                        Response::new(StatusCode::Ok, Some("<h1>It works!</h1>".to_string()))
                    }

                    Err(e) => {
                        println!("Failed to parse request: {}", e);
                        Response::new(StatusCode::BadRequest, None)
                    }
                };

                if let Err(e) = response.send(&mut stream) {
                    println!("Failed to send response: {}", e);
                }
            }

            Err(e) => println!("Error while processing stream: {}", e),
        }
    }
}
