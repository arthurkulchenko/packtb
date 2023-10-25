mod templates;

// use std::fmt::{Display, Formatter};
// use std::fmt;
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
use log::info;

lazy_static! {
    static ref INDEX_PATH: Regex = Regex::new("^/(index\\.html?)?$").unwrap();
    // static ref USERS_PATH: Regex = Regex::new("^/users/(\\d+)$").unwrap();
    // static ref USERS_PATH: Regex = Regex::new("^/users/?$").unwrap();
    static ref USERS_PATH: Regex = Regex::new("^/users$").unwrap();
    static ref USER_PATH: Regex = Regex::new("^/users/((?P<user_id>\\d+?)/?)?$").unwrap();
}

type UserId = u64;

// impl Display for UserId {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// }

#[derive(Debug)]
struct UserData;
type UserDb = Arc<Mutex<Slab<UserData>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting on http://{}", addr);
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
    info!("request: {:?}", status);
    if body.len() > 0 {
        let body = Body::from(Bytes::from(body.to_string()));
        Response::builder().status(status).body(body).unwrap()
    } else {
        Response::builder().status(status).body(Body::from(Bytes::new())).unwrap()
    }
}

async fn routes(request: Request<Body>, user_db: UserDb) -> Result<Response<Body>, Infallible> {
    let resp = match (request.method(), request.uri().path()) {
        (&Method::GET, "/hello_page") => response(StatusCode::OK, templates::HELLO_PAGE),
        (&Method::GET, "/") => response(StatusCode::OK, templates::ROOT),
        (method, path) if (USER_PATH.is_match(path) || USERS_PATH.is_match(path)) => {
            let optional_user_id = USER_PATH.captures(path);
            match (method, optional_user_id) {
                (&Method::GET, params) => {
                    let users = user_db.lock().unwrap();
                    match params {
                        Some(params_id) => {
                            let current_user_id = params_id.name("user_id").unwrap().as_str().parse::<usize>().unwrap();
                            if let Some(current_user) = users.get(current_user_id) {
                                let display_user = format!("Dedug User: {:?}", current_user);
                                response(StatusCode::OK, &templates::USER_PAGE.replace("user_id", &display_user))
                            } else {
                                response(StatusCode::NOT_FOUND, "")
                            }
                        },
                        None => {
                            let all_users = users.iter().map(|(id, _)| id.to_string()).collect::<Vec<String>>().join(", ");
                            response(StatusCode::OK, &templates::USER_PAGE.replace("user_id", &all_users))
                        }
                    }
                }
                (&Method::POST, None) => {
                    let mut users = user_db.lock().unwrap();
                    let id = users.insert(UserData) as UserId;
                    drop(users);
                    response(StatusCode::OK, &templates::USER_PAGE.replace("user_id", &format!("{}", id)))
                }
                (&Method::POST, Some(_)) => response(StatusCode::BAD_REQUEST, ""),
                (&Method::PUT, Some(params_id)) | (&Method::PATCH, Some(params_id)) => {
                    let current_user_id = params_id.name("user_id").unwrap().as_str().parse::<usize>().unwrap();
                    let users = user_db.lock().unwrap();
                    if let Some(current_user) = users.get(current_user_id) {
                        let display_user = format!("Dedug User: {:?}", current_user);
                        return Ok(response(StatusCode::OK, &templates::USER_PAGE.replace("user_id", &display_user)))
                    } else {
                        response(StatusCode::NOT_FOUND, "")
                    }
                },
                (&Method::DELETE, Some(params_id)) => {
                    let current_user_id = params_id.name("user_id").unwrap().as_str().parse::<usize>().unwrap();
                    let mut users = user_db.lock().unwrap();
                    if let Some(current_user) = users.get(current_user_id) {
                        let display_user = format!("Dedug User: {:?}", current_user);
                        users.remove(current_user_id);
                        return Ok(response(StatusCode::OK, &templates::USER_DELETED_PAGE.replace("user_id", &display_user)))
                    } else {
                        response(StatusCode::NOT_FOUND, "")
                    }
                },
                _ => response(StatusCode::METHOD_NOT_ALLOWED, ""),
            }
        },
        _ => response(StatusCode::NOT_FOUND, "")
    };
    Ok(resp)
}
