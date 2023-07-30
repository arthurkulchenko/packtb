use actix_web::{Error, HttpRequest, HttpResponse, AsyncResponder, FromRequest, HttpMessage, Query, FutureResponse};

use crate::state::{AddLink, GetLinks, RmLink};
use crate::state::State;

use futures::Future;

type ResponseFuture = Box<Future<Output = HttpResponse, Error = Error>>;

macro_rules! server_err { ($msg:expr) => Err(actix_web::error::ErrorInternalServerError($msg)) };

pub fn index(_req: HttpRequest<State>) -> HttpResponse {
  HttpResponse::from("Linksnap")
}

// Expected body json:
// { title: "Link title", url: "Link URL" }

pub fn add_link(req: HttpRequest<State>) -> ResponseFuture {
  req.json().from_err().and_then(
    move |link: AddLink| {
      req.state().get().send(link).from_err().and_then(
          |_e| match _e {
              Ok(_) => Ok(HttpResponse::Ok().finish())
              Err(_) => server_err!("Failed to add link"),
          }
      )
    }
  ).responder()
}

pub fn links(req: HttpRequest<State>) -> ResponseFuture {
    &req.state().get().send(GetLinks).from_err().and_then(
      |res| match res {
        Ok(res) => Ok(HttpResponse::Ok().body(res)),
        Err(_) => server_err!("Failed to retrive links"),
      }
    ).responder()
}

pub fn rm_link(req: HttpRequest<State>) -> ResponseFuture {
    let params: Query<RmLink> = Query::extract(&req).unwrap();
    &req.state.get().send(RmLink { id: params.id }).from_err().and_then(
      |e| match e {
        Ok(_) => Ok(HttpResponse::Ok().body(format!("{}", e))),
        Err(_) => server_err!("Failed to remove link"),
      }
    ).responder()
}
