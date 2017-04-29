#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate hyper;
extern crate hyper_native_tls;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate uuid;

mod auth;
mod t411;

use t411::auth::{Login};
use t411::api::{T411Token};

use rocket::{State};
use rocket::http::{Cookie,Cookies};
use rocket::response::{Redirect};
use rocket::request::{Form};
use rocket_contrib::Template;
use std::collections::HashMap;
use std::sync::{Mutex};
use uuid::{Uuid};

type Users = Mutex<HashMap<Uuid, User>>;

#[derive(Debug)]
struct User{
  uuid: Uuid,
  token: T411Token
}

impl User {
  pub fn new(uuid: Uuid, token: T411Token) -> Self {
    User {
      token,
      uuid
    }
  }
}

#[derive(Debug)]
struct AppState{
  pub users: Users
}

impl AppState {
  pub fn new() -> Self {
    AppState {
      users: Mutex::new(HashMap::<Uuid, User>::new())
    }
  }
}

#[get("/")]
fn index(cookies: &Cookies) -> Result<&'static str, Redirect> {
  if !auth::is_authenticated(&cookies){
    return Err(Redirect::to("/login"));
  } else {
    return Ok("Hello world!");
  }
}

#[get("/login")]
fn login() -> Template {
  let context: HashMap<String, String> = HashMap::new();
  Template::render("login", &context)
}

#[post("/login", data = "<login>")]
fn post_login(mut cookies: &Cookies, login: Form<Login>, state: State<AppState>) -> Result<Redirect, Template> {
  let credentials = login.get();

  match t411::auth::authenticate(&credentials) {
    Ok(token) => {
      let uuid = Uuid::new_v4();
      let user = User::new(uuid, token.token);
      let mut users = state.users.lock().unwrap();

      users.insert(uuid, user);

      cookies.add(Cookie::new("session", uuid.to_string()));

      Ok(Redirect::to("/"))
    },
    Err(err) => {
      let context: HashMap<String, String> = HashMap::new();
      Err(Template::render("login", &context))
    }
  }
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index, login, post_login])
    .manage(AppState::new())
    .launch();
}
