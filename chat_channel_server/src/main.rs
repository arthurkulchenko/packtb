use std::time::Duration;
use std::net::TcpStream;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    // PHASE: 1 setting up server
    // Create server
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    // config server
    server.set_nonblocking(true).expect("Failed to initialize non-blocking");
    // Create variable to store clients
    let mut clients: Vec<TcpStream> = vec![];
    // Create channels to send and receive messages
    let (sink, stream) = mpsc::channel::<String>();
    // Runtime for keeping programm run if there is no connections
    loop {
        // PHASE: 2 accepting and handling connections
        // Check for successfully connected clients and deconstruct to a socket and client address
        if let Ok((mut socket, addr)) = server.accept() {//                                                        <-|
            println!("Client {} connected", addr);//                                                                 |
            // Creating a clone of the sink to send to the thread                                                    |
            let sink = sink.clone();//                                                                               |
            // Pushing clone of the socket to a clients vector                                                       |
            clients.push(socket.try_clone().expect("Failed to clone client"));//                                     |
            // Spawning thread with endless loop for reading messages from the client in case we have a connection >-|
            thread::spawn(
                move || loop {
                    // Creating buffer full of '0'
                    let mut buff = vec![0; MSG_SIZE];
                    // Reading from socket to the buffer
                    match socket.read_exact(&mut buff) {
                        // Processing messages
                        Ok(_) => {
                            // Clear out '0's, then dereference and collect to a vector
                            let message = buff.iter().take_while(|&x| *x != 0 ).map(|x| *x).collect::<Vec<_>>();
                            // Convert to a string
                            let msg = String::from_utf8(message).expect("Invalid utf8 message");
                            println!("{}: {:?}", addr, msg);
                            // Send to message to sink.
                            sink.send(msg).expect("Failed to send message to sink");
                            // QUESTION: Doesn't we need to break the loop here?
                        },
                        // Handles errors
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(_) => {
                            println!("Closing connnection with {}", addr);
                            break;
                        }
                    }
                    sleep();
                }
            );
        };
        // PHASE: Reading message
        // Reading messages from the stream
        if let  Ok(msg) = stream.try_recv() {
            clients = clients.into_iter().filter(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).map(|_| client).ok().is_some()
            }).collect::<Vec<_>>();
        }
        sleep();
    }
}

fn sleep() {
    thread::sleep(Duration::from_millis(100));
}
