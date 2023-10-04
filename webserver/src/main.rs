mod server;
mod http;
mod controllers;

use crate::http::*;
use crate::server::Server;
use controllers::Controller;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    // let public_path = "./public".to_string();
    let public_path = std::env::var("PUBLIC_PATH").unwrap();
    let server = Server::new("127.0.0.1:8080");
    server.run(Controller::new(public_path));
}
