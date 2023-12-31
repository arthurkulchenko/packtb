use super::server::Handler;
use super::http::{Request, Response, HttpMethods};

pub struct Controller {
    public_path: String
}

impl Controller {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, path.to_string());
        match std::fs::canonicalize(path) {
            Ok(path) => {
                // path - of PathBuf type
                if path.starts_with(&self.public_path) {
                    std::fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack attempt: {}", path.display());
                    None
                }
            },
            Err(_) => None,
        }
    }

}

impl Handler for Controller {
    fn handle_request(&mut self, request: &Request) -> Response {
        println!("Request: {:?}", request);

        match request.method() {
            HttpMethods::GET => match request.path() {
                "/" => Response::new(200, "OK".to_string(), self.read_file("index.html")),
                "/info" => Response::new(200, "OK".to_string(), self.read_file("info.html")),

                path => match self.read_file(path) {
                    Some(content) => Response::new(200, "OK".to_string(), Some(content)),
                    None => Response::new(404, "Not Found".to_string(), None),
                },

            },
            _ => Response::new(404, "Not Found".to_string(), None),
        }
    }
}

// Response::new(200, "OK".to_string(), Some("handler 1".to_string()))
        // let result = Request::try_from(&buffer[..]);
        // if let Err(_message) = result {
        //     println!("{}", "error message");
        //     return; // Response::new(400, "Bad Request".to_string(), None).send_to(&mut stream)
        // }
        // result.unwrap();
        // let body = std::fs::read_to_string(format!("{}/index.html", self.public_path)).unwrap().to_string();
        // Response::new(code: 200, status: "OK".to_string(), body: Some(body), stream));
        // match write!(stream, "HTTP/1.1 400 Not Found\r\n\r\n") {
        //     Ok(_) => println!("{}", "body"),
        //     Err(e) => println!("{}", e)
        // }
