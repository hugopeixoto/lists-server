#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::r2d2;

pub mod actions;
pub mod db;
pub mod models;
pub mod schema;

pub mod http {
    pub fn internal_server_error<E>(e: E) -> actix_web::HttpResponse
    where
        E: core::fmt::Display,
    {
        println!("internal server error: {}", e);
        actix_web::HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let port = std::env::var("PORT").unwrap_or("3030".to_string());

    let manager = db::ConnectionManager::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            //.route("/", web::get().to(index))
            //.route("/item/{id}", web::patch().to(item_update))
            .route("/collection/{id}", web::get().to(actions::collection_show))
            .route(
                "/collection/{id}/item",
                web::post().to(actions::item_create),
            )
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
