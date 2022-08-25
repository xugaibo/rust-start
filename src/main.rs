extern crate core;

mod apis;
mod cores;
mod db;
mod midlleware;
mod models;
mod service;

use crate::cores::context::Context;
use actix_web::{web, App, HttpServer};
use actix_web::web::ServiceConfig;
use apis::article;
use crate::apis::{token, user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    let db_connect = db::connect::get_connect().await;
    let rust_context = Context { conn: db_connect };

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/")
                    .wrap(midlleware::token_check::TokenCheck)
                    .configure(configure),
            )
            .app_data(web::Data::new(rust_context.clone()))
            .route("/user", web::post().to(user::create))
            .route("/token", web::post().to(token::create))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        "/article",
        web::post().to(article::create),
    );
}
