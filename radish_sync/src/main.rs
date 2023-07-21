use lazy_static::lazy_static;
use resp::Decoder;
use std::collections::HashMap;
use std::env;
use std::io::{Read, Write, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
// use std::thread;

mod commands;
use crate::commands::handle_client_request;

type STORE = Mutex<HashMap<String, String>>;

lazy_static! {
    static ref RADISH_DB: STORE = Mutex::new(HashMap::new());
}

fn main() {
    let addr = env::args().skip(1).next().unwrap_or("127.0.0.1:6378".to_owned());
    let listener = TcpListener::bind(&addr).unwrap();
    println!("RADISH sync listening on {}", addr);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection from {:?}", stream);
        handle_client(stream);
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = vec![0; 512];
    stream.read(&mut buffer).unwrap();

    let reader = BufReader::new(&*buffer);
    let decoder = Decoder::new(reader).decode();
    match decoder {
        Ok(v) => {
            let reply = handle_client_request(v);
            stream.write_all(&reply).unwrap();
        }
        Err(e) => {
            println!("Error: Invalid command: {:?}", e);
            let _ = stream.shutdown(std::net::Shutdown::Both);
        }
    };
}
