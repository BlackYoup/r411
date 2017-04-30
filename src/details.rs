use hyper::method::Method;
use std::collections::HashMap;
use t411::api::{call,T411Error};
use torrent::{Privacy};
use user::{User};
use serde_json;

#[derive(Serialize,Deserialize,Debug)]
pub struct DetailedTorrent {
  pub id: usize,
  pub name: String,
  pub rewritename: String,
  pub category: usize,
  pub categoryname: String,
  pub categoryimage: String,
  pub username: String,
  pub privacy: Privacy,
  pub terms: HashMap<String, String>,
  pub description: String
}

impl DetailedTorrent {
  pub fn get(id: usize, user: User) -> Result<DetailedTorrent, T411Error> {
    let res = call(&format!("/torrents/details/{}", id), Method::Get, Some(user.token), None, None);

    res.map(|(json, _)| serde_json::from_str(&json).expect("detailed torrent: failed to deserialize"))
  }
}
