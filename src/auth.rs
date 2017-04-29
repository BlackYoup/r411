use ::{AppState};
use t411::api::{T411Token};
use rocket::{State};
use rocket::http::{Cookies};
use user::{User};
use uuid::{Uuid};
use std::str::FromStr;

pub fn authenticate(cookies: &Cookies, state: State<AppState>) -> bool {
  let authenticated = is_authenticated(cookies, state);
  if !authenticated && cookies.find("session").is_some() {
    cookies.remove("session");
  }

  authenticated
}

fn is_authenticated(cookies: &Cookies, state: State<AppState>) -> bool {
  cookies
    .find("session")
    .and_then(|cookie| Uuid::from_str(cookie.value()).ok())
    .and_then(|uuid| {
      match state.users.lock().unwrap().contains_key(&uuid) {
        true => Some(true),
        false => None
      }
    })
    .unwrap_or(false)
}

pub fn save_user(state: State<AppState>, token: T411Token) -> User {
  let uuid = Uuid::new_v4();
  let user = User::new(uuid, token);
  let mut users = state.users.lock().unwrap();

  users.insert(uuid, user.clone());

  user
}
