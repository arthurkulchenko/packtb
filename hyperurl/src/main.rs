use log::{info};
use std::env;

use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use tokio::net::TcpListener;

use hyper::body::Bytes;
// use hyper::service::service_fn;
// use hyper::{Request, Response};
use hyper_util::rt::TokioIo;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
// use hyper::body;

use std::sync::RwLock;
use core::future::Future;
use std::collections::HashMap;
use std::sync::{Arc};

use lazy_static::lazy_static;
// use crate::shortener::shorten_url;

type UrlDb = Arc<RwLock<HashMap<String, String>>>;
lazy_static! { static ref SHORT_URLS: UrlDb = Arc::new(RwLock::new(HashMap::new())); }

// async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
//     Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
// }

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "hyperurl=info");
    pretty_env_logger::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(url_service))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    // let listener = TcpListener::bind(addr).await?;
    // loop {
    //     let (stream, _) = listener.accept().await?;
    //     let io = TokioIo::new(stream);
    //     tokio::task::spawn(async move {
    //         if let Err(err) = http1::Builder::new().serve_connection(io, service_fn(url_service)).await {
    //             println!("Error serving connection: {:?}", err);
    //         }
    //     });
    // }
}

async fn url_service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let res = req.into_body();
    info!("Request: {:?}", res);
    // let reply = req.map(move |chunk| {
    //   info!("Request: {:?}", chunk);

    //   // let url_to_shorten = std::str::from_utf8(&chunk).unwrap();
    //   // Now you have the body as a String and you can work with it.
    //   // let shortened_url = shorten_url(url_to_shorten);
    //   // SHORT_URLS.write().unwrap().insert(shortened_url, url_to_shorten.to_string());
    //   // let a = &*SHORT_URLS.read().unwrap();
    //   // Response::new(Full::new(Bytes::from(format!("{:#?}", a))))
    // });


    // let entire_body = body::to_bytes(req.into_body()).await?;

    // println!("Received body: {:?}", entire_body);
   Ok(Response::new("Hello, World!".into()))
}
