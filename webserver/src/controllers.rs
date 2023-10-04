use super::server::Handler;
use super::http::{Request, Response};

pub struct Controller;

impl Handler for Controller {
    fn handle_request(&mut self, request: &Request) -> Response {
        println!("Request: {:?}", request);

        Response::new(200, "OK".to_string(), Some("handler 1".to_string()))
    }
}
