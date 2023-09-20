mod server;
mod http;

use crate::http::*;
use crate::server::Server;

fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.run();
}
