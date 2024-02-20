use serde::{Deserializer, Serialize, Serializer};
use time::Date;

#[derive(serde::Serialize, Debug)]
pub struct GameDetails {
    pub id: String,
    pub name: String,
    #[serde(serialize_with = "serialize_date")]
    pub release_date: Option<Date>,
}

fn serialize_date<S>(date: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    if let Some(date) = date {
        date.to_string().serialize(serializer)
    } else {
        serializer.serialize_none()
    }
}

impl GameDetails {
    pub fn new(id: String, name: String, release_date: Option<Date>) -> GameDetails {
        GameDetails { id, name, release_date }
    }
}

