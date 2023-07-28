use log::{info};
use std::env;
// use hyper::server;
// use hyper::service::service_fn;
use tokio::net::TcpListener;
// use hyper::rt::{self};
// use hyper::server::conn::http1;
// use hyper::{Request, Response};
use http_body_util::Full;
use hyper::body::Bytes;
// use std::convert::Infallible;
// use std::net::SocketAddr;

use std::convert::Infallible;
// use std::net::SocketAddr;
use hyper::{Body, Error, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
// use hyper::server::Builder;

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
    let addr = "127.0.0.1:3000".parse().unwrap();
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

            let make_service = make_service_fn(|_conn_state| async {
                // Ok::<_, Infallible>(service_fn(hello))
                Ok::<_, Error>(service_fn(|req| async {
                    url_service(req).await
                    // url_service(req)
                    // Ok::<_, Error>(Response::new(Body::from("Hello World")))
                }))
            });

            let server = Server::bind(&addr).serve(make_service);


            // if let Err(err) = http1::Builder::new()
            //     // `service_fn` converts our function in a `Service`
            //     // .serve_connection(stream, service_fn(url_service))
            //     .serve_connection(stream, service_fn(hello))
            //     .await { eprintln!("Error serving connection: {:?}", err); }
        });
    }
}

// async fn hello(_: Request<Body>) -> Result<Response<Full<Bytes>>, Infallible> {
async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(
        // Response::new(Full::new(Bytes::from("Hello, World!")))
        Response::new(Body::from("Hello World"))
    )
}

use std::sync::RwLock;
use core::future::Future;
use std::collections::HashMap;
use std::sync::{Arc};
use std::str;
// use hyper::Request;
// use hyper::{Body, Response};
// use hyper::rt::{Fututre};
use futures::future::FutureExt;

use lazy_static::lazy_static;
use crate::shortener::shorten_url;

type UrlDb = Arc<RwLock<HashMap<String, String>>>;
// type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

// type BoxFut = Box<dyn Future<Output = Result<Response<Body>, hyper::Error>> + Send>;
// type BoxFut = Box<dyn Response<Body>, hyper::Error>;

lazy_static! { static ref SHORT_URLS: UrlDb = Arc::new(RwLock::new(HashMap::new())); }

// async fn url_service(req: Request<Body>) -> Box<dyn Future<Output = Result<Response<Body>, hyper::Error>>> {
async fn url_service(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let reply = hyper::body::to_bytes(req.into_body()).await;
    let respond = reply.map(move |chunk| {
    // let reply = req.into_body().concat().map(move |chunk| {
        println!("Request: {:?}", chunk);
        let c = chunk.iter().cloned().collect::<Vec<u8>>();

        let original_url = str::from_utf8(&c).unwrap();
        let shortened_url = shorten_url(original_url);

        SHORT_URLS.write().unwrap().insert(shortened_url, original_url.to_string());
        let all_urls = &*SHORT_URLS.read().unwrap();
        // "{:#?}" pretty print
        Response::new(Body::from(format!("{:#?}", all_urls)))
    });
    // reply
    // Box::new(respond)
    respond
}
