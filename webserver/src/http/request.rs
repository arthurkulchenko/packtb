// use std::str::Utf8Error;
use std::collections::HashMap;
use crate::HttpMethods;
use std::convert::TryFrom;
// use std::error::Error;
// use std::fmt::{Display, Formatter, Debug};

// ========================================================== ERROR
// pub enum ParseError {
//     InvalidRequest,
//     InvalidEncoding,
//     InvalidProtocol,
//     InvalidMethod,
// }

// impl ParseError {
//     fn message(&self) -> &str {
//         match self {
//             Self::InvalidRequest => "Invalid Request",
//             Self::InvalidEncoding => "Invalid Encoding",
//             Self::InvalidProtocol => "Invalid Protocol",
//             Self::InvalidMethod => "Invalid Method",
//         }
//     }
// }

// impl Display for ParseError {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.message())
//     }
// }

// impl Debug for ParseError {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.message())
//     }
// }

// impl From<Utf8Error> for ParseError {
//     fn from(_: Utf8Error) -> Self {
//         Self::InvalidEncoding
//     }
// }

// impl Error for ParseError {}
// ========================================================== ERROR

#[derive(Debug)]
pub struct Request {
    method: HttpMethods,
    path: String,
    // query: Option<String>,
    query: Option<HashMap<String, String>>,
}

impl Request {
    // pub fn from_byte_array(buffer: &[u8]) -> Self {
    // pub fn from_byte_array(buffer: &[u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buffer) {
        //     Ok(request) => { return result },
        //     Err(_) => return Err(ParseError::InvalidEncoding)
        // }
        // match str::from_utf8(buffer).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => { retunr result },
        //     Err(e) => return Err(e)
        // }
    //     let request = str::from_utf8(buffer)?;
    //     unimplemented!()
    // }
}

impl TryFrom<&[u8]> for Request {
    // type Error = ParseError;
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let buffer_read = String::from_utf8_lossy(value);
        let contence: Vec<_> = buffer_read.split("\r\n").collect();
        let contence_vec = contence[0].split(" ").collect::<Vec<_>>();
        // println!("{:?}", contence_vec);
        let (method, path_with_query, _protocol) = (contence_vec[0], contence_vec[1], contence_vec[2]);
        let path_query: Vec<_> = path_with_query.split("?").collect();
        let path_query_len = path_query.len();

        let (p, q) = match path_query_len {
            2 => {
                // let qq = path_query[1].to_string()
                let key_value_pairs = path_query[1].split("&").collect::<Vec<_>>();
                let mut qq = HashMap::new();
                for key_value in key_value_pairs {
                    let key_value_array = key_value.split("=").collect::<Vec<_>>();
                    qq.insert(key_value_array[0].to_string(), key_value_array[1].to_string());
                }
                (path_query[0].to_string(), Some(qq))
            },
            _ => { (path_query[0].to_string(), None) }
        };
        // let (p, q) = (path_query[0], path_query[1]);
        // let http_method = HttpMethods::from_str(method);
        let http_method = method.parse()?;
        // let selfy = Self { method: HttpMethods::GET, path: p.to_string(), query: Some(q.to_string()) };
        let selfy = Self { method: http_method, path: p, query: q };
        Ok(selfy)
    }
}
