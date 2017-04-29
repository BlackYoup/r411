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
mod user;

use t411::auth::{Login};
use user::{User,Users};

use rocket::{State};
use rocket::http::{Cookie,Cookies};
use rocket::response::{Redirect};
use rocket::request::{Form};
use rocket_contrib::Template;
use std::collections::HashMap;
use std::sync::{Mutex};
use uuid::{Uuid};

#[derive(Debug)]
pub struct AppState{
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
fn index(cookies: &Cookies, state: State<AppState>) -> Result<&'static str, Redirect> {
  if !auth::is_authenticated(&cookies, state){
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
fn post_login(cookies: &Cookies, login: Form<Login>, state: State<AppState>) -> Result<Redirect, Template> {
  let credentials = login.get();

  match t411::auth::authenticate(&credentials) {
    Ok(token) => {
      let user = auth::save_user(state, token.token);
      cookies.add(Cookie::new("session", user.uuid.to_string()));
      Ok(Redirect::to("/"))
    },
    Err(_) => {
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
