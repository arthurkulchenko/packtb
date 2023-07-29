use log::{info};
use std::env;

use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

// async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
//     Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env::set_var("RUST_LOG", "hyperurl=info");
    pretty_env_logger::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, service_fn(url_service)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn url_service(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
  // let reply = req.map(move |chunk| {
  //   info!("Request: {:?}", chunk);
  //   let c = chunk.cloned().collect::<Vec<u8>>();
  //   // let url_to_shorten = str::from_utf8(&c).unwrap();
  //   // let shortened_url = shorten_url(url_to_shorten);
  //   // SHORT_URLS.write().unwrap().insert(shortened_url, url_to_shorten.to_string());
  //   // let a = &*SHORT_URLS.read().unwrap();
  //   // Response::new(Full::new(Bytes::from(format!("{:#?}", a))))
  //   info!("Request: {:?}", c);
  // });
    while let Some(chunk) = req.body().cloned().next().await {
        let chunk = chunk?;
        println!("Received chunk: {:?}", chunk);
    }
   Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
