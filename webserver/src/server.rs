use crate::http::*;

use std::net::TcpListener;
use std::io::{Read};
// use std::convert::TryFrom;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, parse_error: &ParseError) -> Response {
    // fn handle_bad_request(&mut self, parse_error: &String) -> Response {
        println!("Failed parse request: {}", parse_error);
        Response::new(400, "Bad Request".to_string(), None)
    }
}

pub struct Server {
    host: String,
    port: String
}

impl Server {
    pub fn new(s: &str) -> Self {
        // let delimiter_index = s.find(":").unwrap();
        // let host = &s[..delimiter_index];
        // let port = &s[delimiter_index - 1..];
        let [host, port] = s.split(":").collect::<Vec<&str>>()[..] else { panic!("Wrong webserver address!") };
        Self { host: host.to_string(), port: port.to_string() }
    }

    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(format!("{}:{}", &self.host, &self.port)).unwrap();
        println!("Server started on {}:{}", self.host, self.port);
        'server_runtime: loop {
            match listener.accept() {
                Ok((mut stream, _address)) => {
                    let buffer = &mut [0; 1024];

                    match stream.read(buffer) {
                        Ok(_req) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(parse_error) => handler.handle_bad_request(&parse_error)
                            };
                            if let Err(error) = response.send_to(&mut stream) {
                               println!("{}", error);
                            }
                        },
                        Err(error) => println!("{}", error)
                    };
                },
                Err(error) => println!("{}", error)
            }
        }
    }
}
