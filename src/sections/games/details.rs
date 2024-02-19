use time::Date;

#[derive(serde::Serialize, Debug)]
pub struct GameDetails {
    id: String,
    name: String,
    release_date: String
}

impl GameDetails {
    pub fn new(id: String, name: String, release_date: Option<Date>) -> GameDetails {
        let release_date = release_date.map_or(String::new(),|date| date.to_string());
        GameDetails { id, name, release_date }
    }
}

