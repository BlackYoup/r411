use t411::api::{call,T411Error,T411Token};
use hyper::method::Method;
use hyper::header::{ContentType,Headers};
use serde_json;

#[derive(FromForm, Serialize)]
pub struct Login{
  username: String,
  password: String
}

#[derive(Deserialize)]
pub struct TokenResponse{
    pub token: T411Token
}

pub fn authenticate(credentials: &Login) -> Result<TokenResponse, T411Error> {
  let body = format!("username={}&password={}", credentials.username, credentials.password);

  let mut headers = Headers::new();
  headers.set(ContentType::form_url_encoded());

  let result = call("/auth", Method::Post, None, Some(&body), Some(headers));

  match result {
    Ok(res) => Ok(serde_json::from_str(&res).expect("authenticate: couldn't deserialize token")),
    Err(e) => Err(e)
  }
}
