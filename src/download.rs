use hyper::client::response::{Response};
use hyper::method::Method;
use rocket::response::{Stream};
use t411::api::{call,T411Error};
use user::{User};

pub fn torrent(id: usize, user: User) -> Result<Stream<Response>, T411Error> {
  let res = call(&format!("/torrents/download/{}", id), Method::Get, Some(user.token), None, None);

  res.map(|(_, response)| Stream::from(response))
}
