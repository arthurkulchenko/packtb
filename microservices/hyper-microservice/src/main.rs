// use hyper::service::service_fn;
// use hyper::{Body, Response, Server};
// use hyper::rt::Future;

use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

// fn main() {
//     let addr = ([127, 0, 0, 1], 8080).into();
//     let builder = Server::bind(&addr);
//     let server = builder.serve(|| {
//         service_fn_ok(|_| {
//             Response::new(Body::from("String"))
//         })
//     });
//     let server = server.map(drop);
//     hyper::rn::run(server);
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;
    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;
        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);
        // Spawn a tokio task to serve multiple connections concurrently
        // Finally, we bind the incoming connection to our `handler` service
        // `service_fn` converts our function in a `Service`
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, service_fn(handler)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn handler(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
