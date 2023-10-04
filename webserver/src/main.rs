mod server;
mod http;
mod controllers;

use crate::http::*;
use crate::server::Server;
use controllers::Controller;
// use dotenv::dotenv;

fn main() {
    let default_statics_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // dotenv().ok();
    // let public_path = "./public".to_string();
    // let public_path = std::env::var("PUBLIC_PATH").unwrap();
    let server = Server::new("127.0.0.1:8080");
    server.run(Controller::new(default_statics_path));
}
