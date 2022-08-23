use crate::models::request::create_article::CreateArticle;
use actix_web::{web, Responder};
use crate::{Context};
use crate::models::response::response::Response;
use crate::service::article;

pub async fn create(req: web::Json<CreateArticle>, data: web::Data<Context>) -> impl Responder {
    let r = article::create(req.0, data).await;
    return Response::from_result(&r).to_json_resp();
}
