use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    // Connect to chat server
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("Failed to initialize non-blocking");
    let (sink, stream) = mpsc::channel::<String>();

    thread::spawn(
        move || loop {
            // Create buffer
            let mut buff = vec![0; MSG_SIZE];
            // READING FROM CLIENT
            // Put message into buffer
            match client.read_exact(&mut buff) {
                Ok(_) => {
                    // Clear out empty zeros and make vector of bytes
                    let msg = buff.into_iter().take_while(|&x| x != 0).map(|x| x).collect::<Vec<_>>();
                    println!("Message received {:?}", String::from_utf8(msg).expect("Invalid utf8 message"));
                },
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    println!("Connection with server was severed");
                    break;
                }
            }
            // READING FROM CHANNEL AND WRITING TO OWN CLIENT
            match stream.try_recv() {
                Ok(msg) => {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MSG_SIZE, 0);
                    // NOTIMCE: This is how we transmit data to other clients trough server, using client connection!!!
                    client.write_all(&buff).expect("Writing to socket failed");
                    // Can be ommited
                    // println!("Message sent {:?}", msg);
                },
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => break
            }

            thread::sleep(Duration::from_millis(100));
        }
    );
    println!("Write a message:");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || sink.send(msg).is_err() { break }
    }
    println!("Bye bye!");
}
