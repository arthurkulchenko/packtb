use crate::Request;
use crate::Response;

use std::net::TcpListener;
use std::io::{Read, Write};
// use std::convert::TryFrom;

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

    pub fn run(self) {
        let listener = TcpListener::bind(format!("{}:{}", &self.host, &self.port)).unwrap();
        println!("Server started on {}:{}", self.host, self.port);
        'server_runtime: loop {
            match listener.accept() {
                Ok((mut stream, _address)) => {
                    let buffer = &mut [0; 1024];

                    match stream.read(buffer) {

                        Ok(_req) => {
                            let result = Request::try_from(&buffer[..]);
                            // println!("{:?}", result);
                            if let Err(_message) = result {
                                println!("{}", "error message");
                                return
                            }

                            result.unwrap();
                            let _response = Response { code: 200, status: "OK".to_string(), body: None };
                            let resp = "HTTP/1.1 200 OK\r\n\r\nHello".to_string();
                            write!(stream, "{}", resp);
                            // match write!(stream, "HTTP/1.1 400 Not Found\r\n\r\n") {
                            //     Ok(_) => println!("{}", "resp"),
                            //     Err(e) => println!("{}", e)
                            // }
                        },
                        Err(error) => println!("{}", error)
                    }
                },
                Err(error) => println!("{}", error)
            }
        }
    }
}
