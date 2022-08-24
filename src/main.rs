extern crate core;

mod apis;
mod cores;
mod db;
mod midlleware;
mod models;
mod service;

use crate::cores::context::Context;
use actix_web::{web, App, HttpServer};
use apis::article;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    let db_connect = db::connect::get_connect().await;
    let rust_context = Context { conn: db_connect };

    HttpServer::new(move || {
        App::new()
            .wrap(midlleware::token_check::TokenCheck)
            .app_data(web::Data::new(rust_context.clone()))
            .route("/article", web::post().to(article::create))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
