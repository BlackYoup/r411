use ::{AppState};
use t411::api::{T411Token};
use rocket::{State};
use rocket::http::{Cookies};
use user::{User};
use uuid::{Uuid};


pub fn is_authenticated(cookies: &Cookies) -> bool {
  match cookies.find("session") {
    Some(_) => true,
    None => false
  }
}

pub fn save_user(state: State<AppState>, token: T411Token) -> User {
  let uuid = Uuid::new_v4();
  let user = User::new(uuid, token);
  let mut users = state.users.lock().unwrap();

  users.insert(uuid, user.clone());

  user
}
