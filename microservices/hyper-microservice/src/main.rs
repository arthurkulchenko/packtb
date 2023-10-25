mod templates;

use std::sync::Arc;
use std::sync::Mutex;
use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::body::Bytes;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};

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
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let user_db = Arc::new(Mutex::new(Slab::new()));
    let make_svc = make_service_fn(|_conn| {
        let user_db = user_db.clone();
        async {
            Ok::<_, Infallible>(service_fn(move |request| {
                let user_db = user_db.clone();
                routes(request, user_db)
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn response(status: StatusCode, body: &str) -> Response<Body> {
    if body.len() > 0 {
        let body = Body::from(Bytes::from(body.to_string()));
        Response::builder().status(status).body(body).unwrap()
    } else {
        Response::builder().status(status).body(Body::from(Bytes::new())).unwrap()
    }
}

async fn routes(request: Request<Body>, user_db: UserDb) -> Result<Response<Body>, Infallible> {
    match (request.method(), request.uri().path()) {
        (&Method::GET, "/hello_page") => Ok(response(StatusCode::OK, templates::HELLO_PAGE)),
        (&Method::GET, "/") => Ok(response(StatusCode::OK, templates::ROOT)),
        // (method, path) if path.starts_with(USER_PATH) => {
        (method, path) if USER_PATH.is_match(path) => {
            let user_id_caps = USER_PATH.captures(path);
            // let user_id = user_id_caps.unwrap().name("user_id").unwrap().as_str();
            let optional_user_id = user_id_caps.unwrap().name("user_id");
            // OPTIONAL: Check user_id is a valid u64
            let mut users = user_db.lock().unwrap();
            match (method, optional_user_id) {
                (&Method::POST, None) => {
                    let _id = users.insert(UserData) as UserId;
                    response(StatusCode::OK, "")
                }
                _ => response(StatusCode::METHOD_NOT_ALLOWED, ""),
            };
            Ok(response(StatusCode::OK, &templates::USER_PAGE.replace("user_id", optional_user_id.unwrap().as_str())))
        },
        _ => Ok(response(StatusCode::NOT_FOUND, ""))
    }
}
