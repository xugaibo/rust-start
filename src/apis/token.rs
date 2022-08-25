use actix_web::{Responder, web};
use validator::Validate;
use crate::Context;
use crate::cores::biz_code::{from_valid_error};
use crate::models::request::create_token::CreateToken;
use crate::models::response::response::Response;
use crate::service::token;

pub async fn create(req: web::Json<CreateToken>, data: web::Data<Context>) -> impl Responder {
    let r = from_valid_error(req.validate());
    if r.is_err() {
        return Response::from_result(&r).to_json_resp();
    }

    let result = token::create(req.0, data).await;
    return Response::from_result(&result).to_json_resp();
}