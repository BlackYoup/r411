use std::clone::{Clone};
use std::collections::{HashMap};
use std::sync::{Mutex};
use t411::api::{T411Token};
use uuid::{Uuid};

pub type Users = Mutex<HashMap<Uuid, User>>;

#[derive(Debug, Clone)]
pub struct User{
  pub uuid: Uuid,
  pub token: T411Token
}

impl User {
  pub fn new(uuid: Uuid, token: T411Token) -> Self {
    User {
      token,
      uuid
    }
  }
}
