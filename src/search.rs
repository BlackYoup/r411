use hyper::method::Method;
use t411::api::{call,T411Error};
use torrent::{Torrents,Torrent,TorrentCategory};
use user::{User};
use serde_json;

#[derive(Deserialize,Debug)]
pub struct SearchTorrent {
  pub id: String,
  pub name: String,
  pub category: String,
  pub seeders: String,
  pub leechers: String,
  pub comments: String,
  pub isVerified: String,
  pub added: String,
  pub size: String,
  pub times_completed: String,
  pub owner: String,
  pub categoryname: String,
  pub categoryimage: String,
  pub privacy: String
}

#[derive(Deserialize,Debug)]
pub struct T411Results {
  pub query: String,
  pub total: String,
  pub offset: usize,
  pub limit: usize,
  pub torrents: Vec<SearchTorrent>
}

#[derive(Debug,Serialize)]
pub struct SearchResults {
  pub query: String,
  pub total: usize,
  pub offset: usize,
  pub limit: usize,
  pub torrents: Torrents
}

#[derive(Debug,FromForm)]
pub struct SearchQS {
  pub q: String,
  pub p: Option<usize>
}

pub struct Search {
  pub query: String,
  pub user: User,
  pub page: Option<usize>,
  pub category: Option<TorrentCategory>,
  pub episode: Option<usize>,
  pub season: Option<usize>
}

impl Search {
  pub fn new(query: String, user: User) -> Self {
    Search {
      query: query,
      user: user,
      page: None,
      category: None,
      episode: None,
      season: None
    }
  }

  pub fn query(&self) -> Result<SearchResults, T411Error>{
    let url = format!("/torrents/search/{}", self.query);
    let res = call(&url, Method::Get, Some(self.user.token.clone()), None, None);

    res
      .map(|(ref json, _)| serde_json::from_str(json).expect("search query: failed to deserialize results"))
      .map(|results: T411Results| SearchResults::from(results))
  }
}

impl From<T411Results> for SearchResults {
  fn from(results: T411Results) -> SearchResults {
    SearchResults {
      query: results.query.to_owned(),
      total: results.total.parse::<usize>().expect("Couldn't parse total"),
      offset: results.offset,
      limit: results.limit,
      torrents: results.torrents.iter().map(|torrent| Torrent::from(torrent)).collect::<Vec<_>>()
    }
  }
}
