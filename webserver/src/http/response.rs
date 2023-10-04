use std::io::Write;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::TcpStream;

// use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    // pub status_code: StatusCode,
    pub code: u16,
    pub status: String,
    pub body: Option<String>
}

impl Response {
    // pub fn new(code: u16, status: StatusCode, body: Option<String>) -> Self {
    pub fn new(code: u16, status: String, body: Option<String>) -> Self {
        Response { code, status, body }
    }

    pub fn send_to(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        write!(stream, "{}", self)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = match &self.body {
            Some(body) => body,
            None => ""
        };
        let code = self.code;
        let status = &self.status;
        write!(f, "HTTP/1.1 {code} {status}\r\n\r\n{body}")
    }
}
