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
// use hyper::body::Body;
use tokio::net::TcpListener;

use slab::Slab;

use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref INDEX_PATH: Regex = Regex::new("^/(index\\.html?)?$").unwrap();
    static ref USERS_PATH: Regex = Regex::new("^/users/(\\d+)$").unwrap();
    // static ref USERS_PATH: Regex = Regex::new("^/users/?$").unwrap()
    static ref USER_PATH: Regex = Regex::new("^/users/((?P<user_id>\\d+?)/?)?$").unwrap();
}

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

// fn response_with_code(status: StatusCode, body: &str) -> Response<Body<Data = Type, Error = Type>> {
// fn response_with_code(status: StatusCode, body: &str) -> Response<dyn Body> {
// fn response_with_code(status: StatusCode, body: &str) -> Response<Full<Bytes>> {
    // Response::builder().status(status).body(Body::empty()).unwrap()
    // ====================================================
    // let response = Response::default();
    // if body.len() > 0 {
    //     response.body_mut().push_str(body);
    // }
    // let (mut parts, body) = response.into_parts();
    // parts.status = status;
    // Response::from_parts(parts, body);
    // Response::new(Full::new(Bytes::from(body)))
// }

fn response_with_code(status: StatusCode) -> Response<Full<Bytes> > {
    Response::builder().status(status).body(Full::new(Bytes::from(""))).unwrap()
}

// let response = Response::default();
//             let (mut parts, body) = response.into_parts();
//             parts.status = StatusCode::NOT_FOUND;
//             Ok(Response::from_parts(parts, body))
// }

async fn routes(request: Request<hyper::body::Incoming>, user_db: UserDb) -> Result<Response<Full<Bytes>>, Infallible> {
    match (request.method(), request.uri().path()) {
        (&Method::GET, "/hello_page") => Ok(Response::new(Full::new(Bytes::from(templates::HELLO_PAGE)))),
        (&Method::GET, "/") => Ok(Response::new(Full::new(Bytes::from(templates::ROOT)))),
        // (method, path) if path.starts_with(USER_PATH) => {
        (method, path) if USER_PATH.is_match(path) => {
            // User specific handlers
            let user_id_caps = USER_PATH.captures(path);
            // let user_id = user_id_caps.unwrap().name("user_id").unwrap().as_str();
            let optional_user_id = user_id_caps.unwrap().name("user_id");
            // OPTIONAL: Check user_id is a valid u64
            let mut users = user_db.lock().unwrap();
            match (method, optional_user_id) {
                (&Method::POST, None) => {
                    let id = users.insert(UserData);
                    // let response = Response::default();
                    response_with_code(StatusCode::OK)
                }
                _ => response_with_code(StatusCode::METHOD_NOT_ALLOWED),
            };
            Ok(Response::new(Full::new(Bytes::from(templates::USER_PAGE.replace("user_id", optional_user_id.unwrap().as_str())))))
        },
        _ => {
            // Ok(response_with_code(StatusCode::NOT_FOUND, ""));
            let response = Response::default();
            let (mut parts, body) = response.into_parts();
            parts.status = StatusCode::NOT_FOUND;
            Ok(Response::from_parts(parts, body))
        }
    }
}
