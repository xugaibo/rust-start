extern crate core;

use actix_web::{App, HttpServer, web};

mod apis;
mod models;
mod cores;

use apis::article;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/article", web::post().to(article::create))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}



