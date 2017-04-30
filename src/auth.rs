use ::{AppState};
use t411::api::{T411Token};
use rocket::{State};
use rocket::http::{Cookies};
use user::{User};
use uuid::{Uuid};
use std::str::FromStr;
use std::env;

pub fn authenticate(cookies: &Cookies, state: State<AppState>) -> Option<User> {
  if env::var("BYPASS_LOGIN").is_ok() {
    save_user(String::new(), &state);
    return get_user(cookies, &state);
  }

  let authenticated = is_authenticated(cookies, &state);
  if !authenticated {
    if cookies.find("session").is_some() {
      cookies.remove("session");
    }

    return None;
  } else {
    return get_user(cookies, &state);
  }
}

fn is_authenticated(cookies: &Cookies, state: &State<AppState>) -> bool {
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

pub fn save_user(token: T411Token, state: &State<AppState>) -> User {
  let uuid;
  let user;
  if env::var("BYPASS_LOGIN").is_ok() {
    uuid = Uuid::from_str(&env::var("UUID").expect("dev mode: couldn't find UUID env")).unwrap();
    user = User::new(uuid, env::var("TOKEN").expect("dev mode: couldn't find TOKEN env"));
  } else {
    uuid = Uuid::new_v4();
    user = User::new(uuid, token);
  }

  let mut users = state.users.lock().unwrap();

  users.insert(uuid, user.clone());

  user
}

pub fn get_user(cookies: &Cookies, state: &State<AppState>) -> Option<User> {
  cookies
    .find("session")
    .and_then(|cookie| Uuid::from_str(cookie.value()).ok())
    .and_then(|uuid| state.users.lock().unwrap().get(&uuid).cloned())
}
