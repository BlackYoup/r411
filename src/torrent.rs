use search::{SearchTorrent};
use serde::{Serialize,Serializer,Deserialize,Deserializer,self};
use serde_json;

#[derive(Debug)]
pub enum Privacy {
  Strong,
  Normal,
  Low
}

pub enum TorrentCategory {
  Series,
  Movie
}

#[derive(Debug,Serialize)]
pub struct Torrent {
  pub id: usize,
  pub name: String,
  pub category: usize,
  pub seeders: usize,
  pub leechers: usize,
  pub comments: usize,
  pub is_verified: bool,
  pub added: String,
  pub size: usize,
  pub times_completed: usize,
  pub owner: usize,
  pub categoryname: String,
  pub categoryimage: String,
  pub privacy: Privacy
}

pub type Torrents = Vec<Torrent>;

impl<'a> From<&'a SearchTorrent> for Torrent {
  fn from(torrent: &SearchTorrent) -> Torrent {
    let is_verified = match torrent.isVerified.as_ref() {
      "1" => true,
      "0" => false,
      _ => unreachable!()
    };

    Torrent {
      id: torrent.id.parse::<usize>().expect("Couldn't parse id"),
      name: torrent.name.to_owned(),
      category: torrent.category.parse::<usize>().expect("Couldn't parse category"),
      seeders: torrent.seeders.parse::<usize>().expect("Couldn't parse seeders"),
      leechers: torrent.leechers.parse::<usize>().expect("Couldn't parse leechers"),
      comments: torrent.comments.parse::<usize>().expect("Couldn't parse comments"),
      is_verified: is_verified,
      added: torrent.added.to_owned(),
      size: torrent.size.parse::<usize>().expect("Couldn't parse size"),
      times_completed: torrent.times_completed.parse::<usize>().expect("Couldn't parse times_completed"),
      owner: torrent.owner.parse::<usize>().expect("Couldn't parse owner"),
      categoryname: torrent.categoryname.to_owned(),
      categoryimage: torrent.categoryimage.to_owned(),
      privacy: Privacy::from(torrent.privacy.to_owned())
    }
  }
}

impl From<String> for Privacy {
  fn from(privacy: String) -> Privacy {
    match privacy.as_ref() {
      "strong" => Privacy::Strong,
      "normal" => Privacy::Normal,
      "low" => Privacy::Low,
      _ => unimplemented!()
    }
  }
}

impl Serialize for Privacy {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
  {
    match *self {
      Privacy::Strong => serializer.serialize_unit_variant("Privacy", 0, "strong"),
      Privacy::Normal => serializer.serialize_unit_variant("Privacy", 1, "normal"),
      Privacy::Low => serializer.serialize_unit_variant("Privacy", 2, "lown")
    }
  }
}

impl Deserialize for Privacy {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer
  {
    let res: serde_json::Value = try!(serde::Deserialize::deserialize(deserializer));
    match res {
      serde_json::Value::String(ref s) if &*s == "strong" => Ok(Privacy::Strong),
      serde_json::Value::String(ref s) if &*s == "normal" => Ok(Privacy::Normal),
      serde_json::Value::String(ref s) if &*s == "low" => Ok(Privacy::Low),
      _ => Err(serde::de::Error::custom("Unexpected Privacy value"))
    }
  }
}
