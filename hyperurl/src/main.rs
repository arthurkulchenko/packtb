use log::{info};
use std::env;
// use hyper::server;
use hyper::service::service_fn;
use tokio::net::TcpListener;
// use hyper::rt::{self};
use hyper::server::conn::http1;
use hyper::{Request, Response};
use http_body_util::Full;
use hyper::body::Bytes;
use std::convert::Infallible;
// use std::net::SocketAddr;

mod shortener;
mod service;

// use crate::service::url_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
// #[tokio::main]
// async fn main() {
    env::set_var("RUST_LOG", "hyperurl=info");
    pretty_env_logger::init();

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    let addr = "127.0.0.1:3002".parse().unwrap();
    // let server = Server::bind(&addr).serv(|| service_fn(url_service)).map_err(|e| error!("server error: {}", e));
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;
    info!("URL shortener listening on http://{}", addr);
    // rt::run(server);
    // server.await;
    loop {
        let (stream, _) = listener.accept().await?;
        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                // .serve_connection(stream, service_fn(url_service))
                .serve_connection(stream, service_fn(hello))
                .await
            { println!("Error serving connection: {:?}", err); }
        });
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
