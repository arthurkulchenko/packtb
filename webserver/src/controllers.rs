use super::server::Handler;
use super::http::{Request, Response, HttpMethods};

pub struct Controller {
    public_path: String
}

impl Controller {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
}

impl Handler for Controller {
    fn handle_request(&mut self, request: &Request) -> Response {
        println!("Request: {:?}", request);

        // Response::new(200, "OK".to_string(), Some("handler 1".to_string()))
        // let result = Request::try_from(&buffer[..]);
        // if let Err(_message) = result {
        //     println!("{}", "error message");
        //     return; // Response::new(400, "Bad Request".to_string(), None).send_to(&mut stream)
        // }
        // result.unwrap();
        let body = r#"
            <html>
                <head>
                    <link href="/main.css" rel="stylesheet">
                    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-EVSTQN3/azprG1Anm3QDgpJLIm9Nao0Yz1ztcQTwFspd3yD65VohhpuuCOmLASjC" crossorigin="anonymous">
                </head>
                <body>
                    <h1>This is it!</h1>
                </body>
            </html>
        "#.to_string();
        let okr_resp = Response::new(200, "OK".to_string(), Some(body));
        // Response::new(code: 200, status: "OK".to_string(), body: Some(body), stream));
        // match write!(stream, "HTTP/1.1 400 Not Found\r\n\r\n") {
        //     Ok(_) => println!("{}", "body"),
        //     Err(e) => println!("{}", e)
        // }
        match request.method() {
            HttpMethods::GET => match request.path() {
                "/" => okr_resp,
                "/main.css" => {
                    let statics = std::fs::read_to_string(format!("{}/main.css", &self.public_path)).unwrap();
                    Response::new(200, "OK".to_string(), Some(statics.to_string()))
                },
                _ => okr_resp,
            },
            _ => Response::new(404, "Not Found".to_string(), None),
        }
    }
}
