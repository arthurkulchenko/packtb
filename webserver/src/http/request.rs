use crate::HttpMethods;
use std::convert::TryFrom;

pub struct Request {
    method: HttpMethods,
    path: String,
    query: Option<String>,
}

impl Request {
    pub fn from_byte_array(buffer: &[u8]) -> Self {
        unimplemented!()
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let buffer_read = String::from_utf8_lossy(value);
        let contence: Vec<_> = buffer_read.split("\r\n").collect();
        let contence_vec = contence[0].split(" ").collect::<Vec<_>>();

        let selfy = Self { method: HttpMethods::GET, path: "/".to_string(), query: None };
        Ok(selfy)
    }
}
