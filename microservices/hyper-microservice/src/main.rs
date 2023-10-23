mod templates;

use std::sync::Arc;
use std::sync::Mutex;
use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Method, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use slab::Slab;

type UserId = u64;
struct UserData;
type UserDb = Arc<Mutex<Slab<UserData>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;
    let user_db = Arc::new(Mutex::new(Slab::new()));
    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;
        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);
        // Spawn a tokio task to serve multiple connections concurrently
        // Finally, we bind the incoming connection to our `handler` service
        // `service_fn` converts our function in a `Service`
        let user_db = user_db.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(
                io,
                service_fn(move |request| {
                    let user_db = user_db.clone();
                    routes(request, user_db)
                }),
            ).await {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn routes(request: Request<hyper::body::Incoming>, user_db: UserDb) -> Result<Response<Full<Bytes>>, Infallible> {
    match (request.method(), request.uri().path()) {
        // Routes
        (&Method::GET, "/hello_page") => Ok(Response::new(Full::new(Bytes::from(templates::HELLO_PAGE)))),
        (&Method::GET, "/") => Ok(Response::new(Full::new(Bytes::from(templates::ROOT)))),
        _ => {
            let response = Response::new("NOT FOUND".into());
            let (mut parts, body) = response.into_parts();
            parts.status = StatusCode::NOT_FOUND;
            Ok(Response::from_parts(parts, body))
        }
    }
}
