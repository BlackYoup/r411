use hyper::client::Client;
use hyper::header::{Authorization,Headers};
use hyper::method::Method;
use hyper::net::{HttpsConnector};
use hyper_native_tls::{NativeTlsClient};
use hyper::status::StatusCode;
use hyper::{Url};
use serde_json;
use std::fmt;

use std::io::Read;

pub type T411Token = String;

// TODO: config file
const T411_API: &str = "https://api.t411.ai";

#[derive(Deserialize)]
pub struct T411Error {
  error: String,
  code: u32
}

impl fmt::Debug for T411Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Error: {}, code: {}", self.error, self.code)
  }
}

// TODO: return an error etheir matching a T411Error or an std::io::Error (not sure if possible)
pub fn call(route: &str, method: Method, token: Option<T411Token>, data: Option<&str>, custom_headers: Option<Headers>) -> Result<String, T411Error> {
  let ssl = NativeTlsClient::new().expect("call: couldn't create new NativeTlsClient");
  let connector = HttpsConnector::new(ssl);
  let client = Client::with_connector(connector);

  let url = Url::parse(&(T411_API.to_owned() + route)).expect("api_call: couldn't parse route");

  let mut req = match method {
    Method::Get =>  client.get(url),
    Method::Post => client.post(url),
    _ => unimplemented!()
  };

  if let Some(headers) = custom_headers {
    req = req.headers(headers);
  }

  if let Some(t411_token) = token {
    req = req.header(Authorization(t411_token.to_owned()));
  }

  if let Some(body) = data {
    req = req.body(body);
  }

  let mut res = req.send().expect("call: couldn't send request to server");
  let mut buff = String::with_capacity(2048);
  // TODO: handle the 0 case
  let _ = res.read_to_string(&mut buff).expect("call: couldn't read_to_string response");

  println!("Return code: {:?}", res.status);

  if res.status >= StatusCode::Ok && res.status < StatusCode::BadRequest {
    return match serde_json::from_str(&buff) {
      Ok(error) => Err(error),
      Err(_) => Ok(buff)
    };
  } else {
    // TODO: I'm not sur we will ever receive an error of the format we expect here
    // since the API is not RESTful
    let error: T411Error = serde_json::from_str(&buff).expect("call: couldn't deserialize T411Error");
    return Err(error);
  }
}
