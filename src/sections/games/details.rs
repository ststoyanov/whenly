use time::Date;

#[derive(serde::Serialize, Debug)]
pub struct GameDetails {
    name: String,
    release_date: String
}

impl GameDetails {
    pub fn new(name: String, release_date: Option<Date>) -> GameDetails {
        let release_date = release_date.map_or(String::new(),|date| date.to_string());
        GameDetails { name, release_date }
    }
}

