use std::error::Error;
use actix_web::http::header::HeaderValue;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use serde_json::Number;
use time::{Date, Month};
use time::macros::{format_description};
use crate::sections::games::details::{GameDetails};

const BASE_URL: &str = "https://www.giantbomb.com/api";
const API_KEY: &str = "410d35a4c4d3825c71b11fb1831ebb70512055cc";
const USER_AGENT: HeaderValue = HeaderValue::from_static("whenly");
const FIELDS: [&str; 6] = ["id", "name", "original_release_date", "expected_release_day", "expected_release_month", "expected_release_year"];

#[derive(serde::Deserialize, Debug)]
pub struct GetByIdResponse {
    results: BombResult
}

#[derive(serde::Deserialize)]
pub struct SearchResponse {
    results: Vec<BombResult>
}

impl Into<GameDetails> for BombResult {
    fn into(self) -> GameDetails {
        let results = self;
        println!("{:?}", results);
        let id = results.id.to_string();
        let name = results.name;
        let format = format_description!("[year]-[month]-[day]");
        let release_date = if let Some(date) = results.original_release_date {
            Date::parse(&date, format).ok()
        } else if let (Some(year), Some(month), Some(day))
            = (results.expected_release_year, results.expected_release_month, results.expected_release_day) {
            Date::from_calendar_date(year as i32, Month::try_from(month as u8).unwrap(), day as u8).ok()
        } else { None };

        GameDetails::new(id, name, release_date)
    }
}
#[derive(Deserialize, Serialize, Debug, Clone)]
struct BombResult {
    id: Number,
    name: String,
    expected_release_day: Option<u32>,
    expected_release_month: Option<u32>,
    expected_release_year: Option<u32>,
    original_release_date: Option<String>
}

pub async fn get_game_by_id(id: &str) -> Result<GameDetails, Box<dyn Error>> {
    let link = format!("{}/game/{}/?api_key={}&format=json&field_list={}",
                       BASE_URL, id, API_KEY, FIELDS.join(","));

    println!("{}", link);

    let details = build_client()
        .get(link)
        .send()
        .await?
        .json::<GetByIdResponse>()
        .await?
        .results
        .into();

    Ok(details)
}

pub async fn get_multiple_games_by_id(ids: Vec<&str>) -> Result<Vec<GameDetails>, Box<dyn Error>> {
    let link = format!("{}/games/?api_key={}&format=json&field_list={}&filter=id:{}",
                       BASE_URL, API_KEY, FIELDS.join(","), ids.join("|"));

    println!("{}", link);

    let details = build_client()
        .get(link)
        .send()
        .await?
        .json::<SearchResponse>()
        .await?
        .results
        .into_iter()
        .map(|r| r.into())
        .collect();

    Ok(details)
}

pub async fn search_games(name: &str) -> Result<Vec<GameDetails>, Box<dyn Error>> {
    let link = format!("{}/search/?api_key={}&format=json&field_list={}&query={}&resources=game",
                       BASE_URL, API_KEY, FIELDS.join(","), name);

    let details = build_client()
        .get(link)
        .send()
        .await?
        .json::<SearchResponse>()
        .await?
        .results
        .into_iter()
        .map(|r| r.into())
        .collect();

    Ok(details)
}

fn build_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert("User-Agent", USER_AGENT);

    Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}
