use actix_web::{App, HttpServer, middleware};
use crate::sections::games::repository::initialize_game_repository;

mod sections;
mod api;
mod collection;
mod details;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    initialize_game_repository().expect("ops");
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(sections::games::api::game_by_id)
            .service(sections::games::api::games_by_ids)
            .service(sections::games::api::search)
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
