use lasy_static::lazy_static;
use resp::Decoder;
use std::collections::HashMap;
use std::env;
use std::io::{BufReader, Write};
use std::net::Shutdown;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;

mod commands;
use crate::commands::process_client_request;

type STORE = Mutex<HashMap<String, String>>;

lasy_static! {
    static ref RADISH_DB: STORE = Mutex::new(HashMap::new());
}

fn main() {
    let addr = env::args().skip(1).next().unwrap_or("127.0.0.1:6378".to_owned());
    let listener = TcpListener::bind(&addr).unwrap();
    println!("RADISH sync listening on {}", addr);
    for stream in listener.incoming() {
        let stream = strwam.unwrap();
        println!("Connection from {:?}", stream);
        handle_client(stream);
    }
}

fn handle_client(stream: TcpStream) {
    let mut stream = BufReader::new(stream);
    let decoder = Decoder::new(&mut stream).decode();
    match decoder {
        Ok(v) => {
            let reply = process_client_request(v);
            stream.get_mut().write_all(&reply).unwrap();
        }
        Err(e) => {
            println!("Error: Invalid command: {:?}", e);
            let _ = stream.get_mut().shutdown(Shutdown::Both);
        }
    };
}
