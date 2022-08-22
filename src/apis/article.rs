use actix_web::{Responder, web};
use crate::cores::api;
use crate::models::request::create_article::CreateArticle;

pub async fn create(req: web::Json<CreateArticle>) -> impl Responder {
    return api::handle(|| req.check());
}

