#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate hyper;
extern crate hyper_native_tls;
#[macro_use] extern crate mime;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate uuid;

mod auth;
mod details;
mod download;
mod search;
mod t411;
mod torrent;
mod user;

use t411::auth::{Login};
use user::{User,Users};
use search::{SearchResults};

use hyper::client::response::{Response};
use rocket::{Request,State};
use rocket::http::{Cookie,Cookies,ContentType};
use rocket::http::ascii::{UncasedAscii};
use rocket::http::hyper::header::{Charset,ContentDisposition,DispositionParam,DispositionType};
use rocket::response::{Redirect,Responder,Stream};
use rocket::request::{Form};
use rocket_contrib::Template;
use std::collections::HashMap;
use std::sync::{Mutex};
use std::borrow::{Cow};
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
fn index(cookies: &Cookies, state: State<AppState>) -> Result<Template, Redirect> {
  let user = auth::authenticate(&cookies, state);

  if user.is_none() {
    return Err(Redirect::to("/login"));
  }

  let mut context = HashMap::new();
  context.insert("title", "Index");

  Ok(Template::render("index", &context))
}

#[get("/login")]
fn login() -> Template {
  let mut context = HashMap::new();
  context.insert("title", "Login");
  Template::render("login", &context)
}

#[get("/search?<query>")]
fn search(query: search::SearchQS, cookies: &Cookies, state: State<AppState>) -> Result<Template, Redirect> {
  let user = auth::authenticate(&cookies, state);

  if user.is_none() {
    return Err(Redirect::to("/login"));
  }

  let search = search::Search::new(query.q, user.unwrap());
  let results = search.query().expect("Query failed");
  // TODO: handle query error and title
  Ok(Template::render("search", &results))
}

#[post("/login", data = "<login>")]
fn post_login(cookies: &Cookies, login: Form<Login>, state: State<AppState>) -> Result<Redirect, Template> {
  let credentials = login.get();

  match t411::auth::authenticate(&credentials) {
    Ok(token) => {
      let user = auth::save_user(token.token, &state);
      cookies.add(Cookie::new("session", user.uuid.to_string()));
      Ok(Redirect::to("/"))
    },
    Err(_) => {
      let mut context = HashMap::new();
      context.insert("title", "Login");

      Err(Template::render("login", &context))
    }
  }
}

#[get("/torrent/<id>")]
fn torrent(cookies: &Cookies, id: usize, state: State<AppState>) -> Result<Template, Redirect> {
  let user = auth::authenticate(&cookies, state);

  if user.is_none() {
    return Err(Redirect::to("/login"));
  }

  let torrent = details::DetailedTorrent::get(id, user.unwrap()).expect("Detailed torrent failed");

  // TODO: handle query error and title
  Ok(Template::render("torrent", &torrent))
}

#[get("/download/<id>")]
fn download<'r>(cookies: &Cookies, id: usize, state: State<AppState>) -> Result<rocket::response::Response<'r>, Redirect> {
  let user = auth::authenticate(&cookies, state);

  if user.is_none() {
    return Err(Redirect::to("/login"));
  }

  // for file name
  let torrent = details::DetailedTorrent::get(id, user.clone().unwrap()).expect("Detailed torrent failed");
  let torrent_file = download::torrent(id, user.unwrap()).expect("downloaded file failed");

  let mut response = torrent_file.respond().expect("Couldn't generate response from torrent_file");

  let content_type = ContentType{
    ttype: UncasedAscii{ string: Cow::Borrowed("application") },
    subtype: UncasedAscii{ string: Cow::Borrowed("x-bittorrent") },
    params: None
  };

  let torrent_file_name = String::from(torrent.name + ".torrent").into_bytes();
  let content_disposition = ContentDisposition {
    disposition: DispositionType::Attachment,
    parameters: vec![DispositionParam::Filename(Charset::Ext("binary".to_owned()), None, torrent_file_name)]
  };

  response.set_header(content_type);
  response.set_header(content_disposition);

  Ok(response)
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index, login, post_login, search, torrent, download])
    .manage(AppState::new())
    .launch();
}
