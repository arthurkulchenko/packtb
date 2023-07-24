mod codec;
mod commands;

use crate::codec::RespCodec;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;
// use std::future::Future;
use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
// use tokio_util::codec::Framed;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio_codec::Decoder;
use std::env;

use crate::commands::handle_client_request;

lazy_static! {
    static ref RADISH_DB: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args().skip(1).next().unwrap_or("127.0.0.1:6378".to_owned());
    let addr = addr.parse::<SocketAddr>()?;
    let listener = TcpListener::bind(&addr).await?;
    println!("RADISH async listening of: {}", addr);

    // let server_future = listener.incoming
    //                             .map_err(|e| eprintln!("failed to accept socket; error = {:?}", e))
    //                             .for_each(handle_client);
    // tokio::run(server_future);
    while let Ok((socket, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("failed to handle client; error = {:?}", e);
            }
        });
    }
    Ok(())
}

async fn handle_client(client: TcpStream) -> Result<(), ()> {
    // let (tx, rx) = Framed::new(client, RespCodec);
    // let mut framed_stream = Framed::new(stream, RespCodec::new());
    let (tx, rx) = RespCodec.framed(client).split();
    let reply = rx.and_then(handle_client_request);
    let task = tx.send_all(reply).then(|res| {
        if let Err(e) = res { println!("failed to process connection; error = {:?}", e); }
        Ok(())
    });
    tokio::spawn(task);
    Ok(())
}
