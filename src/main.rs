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

mod auth;
mod t411;

use t411::auth::{Login};

use rocket::http::{Cookies};
use rocket::response::{Redirect};
use rocket::request::{Form};
use rocket_contrib::Template;
use std::collections::HashMap;

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
fn post_login(login: Form<Login>) -> String {
  let credentials = login.get();

  match t411::auth::authenticate(&credentials) {
    Ok(token) => token.token,
    Err(err) => format!("{:?}", err)
  }
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index, login, post_login])
    .launch();
}
