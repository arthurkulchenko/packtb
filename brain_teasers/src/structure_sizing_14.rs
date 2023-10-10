// TODO: not works
use std::mem::size_of;

struct VeryImportantMessage {
  _message_type: u8,
  _destination: u16
}

pub fn call() {
  println!("VeryImportantMessage occupies {} bytes.", size_of::VeryImportantMessage())
}
