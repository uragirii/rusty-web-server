use std::io::{Read, Write};
use std::net::TcpListener;

use crate::http::header::Header;
use crate::http::response::Response;

use super::http::request::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn listen(&self) {
        let stream = TcpListener::bind(&self.addr).unwrap();
        println!("Server is listening on {}", self.addr);

        loop {
            match stream.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let request = Request::try_from(&buffer[..]).unwrap();
                            println!("{}",request);
                            let mut responseHeaders = Header::new();
                            responseHeaders.insert("TEST".to_string(), "It works");
                            let response = Response::new(crate::http::response_code::ResponseCodes::Ok,responseHeaders, "<h1>This works</>" );
                            write!(stream, "{}", response);
                        }
                        Err(_) => println!("Error reading"),
                    }
                }
                Err(_) => println!("Something went wrong"),
            }
        }
    }
}
