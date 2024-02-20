use rusqlite::{Connection, named_params, Result};
use serde_json::Number;
use time::Date;
use time::macros::format_description;
use uuid::Uuid;
use crate::details::GameDetails;
struct Game {
    id: Uuid,
    giant_bomb_id: Number,
    name: String,
    release_date: Option<Date>,
}

pub fn initialize_game_repository() -> Result<()> {
    let conn = Connection::open("whenly.db")?;
    conn.execute(
        "CREATE TABLE if not exists game (
            id   TEXT PRIMARY KEY,
            giant_bomb_id INTEGER,
            name TEXT NOT NULL,
            release_date TEXT
        )",
        ()
    )?;
    Ok(())
}

pub fn insert_from_giant_bomb(game: &GameDetails) -> Result<()>{
    let conn = Connection::open("whenly.db")?;
    let id = Uuid::new_v4();
    
    conn.execute(
        "INSERT INTO game (id, giant_bomb_id, name, release_date) values (?1, ?2, ?3, ?4)",
        (id.to_string(), &game.id, &game.name, &game.release_date.map(|d| d.to_string()))
    )?;

    Ok(())
}

pub fn get_by_id(id: Uuid) -> Option<GameDetails> {
    let conn = Connection::open("whenly.db").ok()?;

    let mut receiver = conn
        .prepare("SELECT giant_bomb_id, name, release_date FROM game WHERE id = :id;")
        .expect("receiver failed");
    let mut rows = receiver
        .query(named_params!{ ":id": id.to_string() })
        .expect("rows failed");


    if let Ok(Some(row)) = rows.next() {
        let giant_bomb_id: String = row.get(0).ok()?;
        let name: String = row.get(1).ok()?;
        let release_date: Option<String> = row.get(2).ok()?;
        let format = format_description!("[year]-[month]-[day]");

        let release_date = release_date
            .and_then(|date_str| Date::parse(date_str.as_str(), &format).ok());

        Some(GameDetails {
            id: giant_bomb_id,
            name,
            release_date
        })
    } else { None }
}