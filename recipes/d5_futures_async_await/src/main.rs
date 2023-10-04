// WIP
extern crate d5_futures_async_await;

use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
// use tokio::io::ReadBuf;
// use tokio::prelude::*

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let addr: std::net::SocketAddr = "127.0.0.1:8092".parse()?;
    // println!("Listening on: {}", addr);
    print!("Listening on: {}", addr);
    let listener = TcpListener::bind(&addr).await?;
    loop {
        let (mut rw_socket, _socket_addr) = listener.accept().await?;
        // let mut rs = d5_futures_async_await::ReadStream::new(rw_socket);
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                // let n = match rw_socket.read(&mut rs).await {
                let n = match rw_socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                // Write the data back
                if let Err(e) = rw_socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
