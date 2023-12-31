use crate::links::Links;

use std::sync::{Arc, Mutex};

use actix::{Actor, Addr, SyncContext, SyncArbiter, Message, Handler};
use actix_web::{error, Error};

use serde_derive::{Serialize, Deserialize};

const DB_THREADS: usize = 3;

#[derive(Clone)]
pub struct Db {
  pub inner: Arc<Mutex<Links>>
}

impl Db {
  pub fn new(s: Arc<Mutex<Links>>) -> Db {
    Db { inner: s }
  }
}

impl Actor for Db {
  type Context = SyncContext<Self>;
}

impl Handler<GetLinks> for Db {
  type Result = Result<String, Error>;

  fn handle(&mut self, _new_link: GetLinks, _: &mut Self::Context) -> Self::Result {
    Ok(self.inner.lock().unwrap().links())
  }
}

#[derive(Clone)]
pub struct State {
  pub inner: Addr<Db>,
}

impl State {
  pub fn init() -> Self {
    State {
      inner: SyncArbiter::start(
        DB_THREADS,
        move || Db::new(Arc::new(Mutex::new(Links::new())).clone())),
    }
  }

  pub fn get(&self) -> &Addr<Db> {
    &self.inner
  }
}

pub struct GetLinks;

impl Message for GetLinks {
  type Result = Result<String, Error>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddLink {
  pub title: String,
  pub url: String
}

impl Message for AddLink {
  type Result = Result<(), Error>;
}

impl Handler<AddLink> for Db {
  type Result = Result<(), Error>;

  fn handle(&mut self, new_link: AddLink, _: &mut Self::Context) -> Self::Result {
    let mut db_ref = self.inner.lock().unwrap();
    db_ref.add_link(new_link);
    Ok(())
  }
}

#[derive(Serialize, Deserialize)]
pub struct RmLink {
  pub id: LinkId,
}

impl Message for RmLink {
  type Result = Result<usize, Error>;
}

impl Handler<RmLink> for Db {
  type Result = Result<usize, Error>;
  fn handle(&mut self, link: RmLink, _: &mut Self::Context) -> Self::Result {
    let db_ref = self.get_conn()?;
    Link::rm_link(link.id, db_ref()).map_err(|_| error::ErrorInternalServerError("Failed to remove link"))
  }
}
