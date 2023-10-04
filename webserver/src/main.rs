mod server;
mod http;
mod controllers;

use crate::http::*;
use crate::server::Server;
use controllers::Controller;

fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.run(Controller{});
}
