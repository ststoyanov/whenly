use actix_web::{get, HttpResponse, web};
use actix_web::http::header::ContentType;
use crate::sections::games::giant_bomb::{get_game_by_id, get_multiple_games_by_id, search_games};

#[get("/game/{id}")]
pub async fn game_by_id(path: web::Path<String>) -> HttpResponse {
    match get_game_by_id(&path.into_inner()).await {
        Ok(details) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(details),
        Err(_) => HttpResponse::NoContent()
            .content_type(ContentType::json())
            .await
            .unwrap()
    }
}

#[derive(serde::Deserialize)]
pub struct IdParams {
    id: String
}

#[get("/games")]
pub async fn games_by_ids(params: web::Query<IdParams>) -> HttpResponse {
    let game_ids: Vec<&str> = params.id.split(",").collect();

    match get_multiple_games_by_id(game_ids).await {
        Ok(details) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(details),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .await
            .unwrap(),
    }
}

#[derive(serde::Deserialize)]
pub struct SearchParams {
    query: String
}

#[get("/games/search")]
pub async fn search(params: web::Query<SearchParams>) -> HttpResponse {
    match search_games(&params.query).await {
        Ok(details) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(details),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .await
            .unwrap()
    }
}