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

                    let response = match stream.read(buffer) {
                        Ok(_req) => {
                            let result = Request::try_from(&buffer[..]);
                            // if let Err(_message) = result {
                            //     println!("{}", "error message");
                            //     return; // Response::new(400, "Bad Request".to_string(), None).send_to(&mut stream)
                            // }
                            result.unwrap();
                            let body = r#"
                                <html>
                                    <head>
                                        <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-EVSTQN3/azprG1Anm3QDgpJLIm9Nao0Yz1ztcQTwFspd3yD65VohhpuuCOmLASjC" crossorigin="anonymous">
                                    </head>
                                    <body>
                                        <h1>This is it!</h1>
                                    </body>
                                </html>
                            "#.to_string();
                            Response::new(200, "OK".to_string(), Some(body))
                            // Response::new(code: 200, status: "OK".to_string(), body: Some(body), stream));
                            // match write!(stream, "HTTP/1.1 400 Not Found\r\n\r\n") {
                            //     Ok(_) => println!("{}", "body"),
                            //     Err(e) => println!("{}", e)
                            // }
                        },
                        Err(error) => {
                            println!("Error: {}", error);
                            Response::new(400, "Bad Request".to_string(), None)
                        }
                    };
                    if let Err(error) = response.send_to(&mut stream) {
                        println!("{}", error);
                    }
                },
                Err(error) => println!("{}", error)
            }
        }
    }
}
