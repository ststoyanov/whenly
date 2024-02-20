use actix_web::{get, HttpResponse, post};
use actix_web::http::header::ContentType;
use serde::Deserialize;
use crate::collection::personal_collection::get_collection;

#[get("/collection/games")]
pub async fn get_collection_games() -> HttpResponse {
    let collection = get_collection("user");
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(collection)
}

#[derive(Deserialize)]
struct GameRequest {

}

#[post("/collection/games")]
pub async fn add_game() -> HttpResponse {
    let collection = get_collection("user");
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(collection)
}

