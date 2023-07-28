// use std::sync::RwLock;
// use core::future::Future;
// use std::collections::HashMap;
// use std::sync::{Arc};
// use std::str;
// use hyper::{Body, Error, Request, Response};
// // use hyper::rt::{Fututre};
// use futures::future::FutureExt;

// use lazy_static::lazy_static;
// use crate::shortener::shorten_url;

// type UrlDb = Arc<RwLock<HashMap<String, String>>>;
// // type BoxFut = Box<dyn Output = Response<Body>, hyper::Error> + Send;

// lazy_static! { static ref SHORT_URLS: UrlDb = Arc::new(RwLock::new(HashMap::new())); }

// pub(crate) fn url_service(req: Request<Body>) -> BoxFut {
//   let reply = req.into_body().concat2().map(move |chunk| {
//     println!("Request: {:?}", chunk);
//     // cloned ??
//     // iter + collect ??
//     let c = chunk.iter().cloned().collect::<Vec<u8>>();
//     let url_to_shorten = str::from_utf8(&c).unwrap();
//     let shortened_url = shorten_url(url_to_shorten);
//     SHORT_URLS.write().unwrap().insert(shortened_url, url_to_shorten.to_string());
//     let a = &*SHORT_URLS.read().unwrap();
//     // "{:#?}" pretty print
//     Response::new(Body::from(format!("{:#?}", a)))
//   });
//   Box::new(reply)
// }
