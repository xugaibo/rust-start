use actix_web::{Responder, web};
use validator::Validate;
use crate::Context;
use crate::cores::biz_code::from_valid_error;
use crate::models::request::create_user::CreateUser;
use crate::models::response::response::Response;
use crate::service::user;

pub async fn create(req: web::Json<CreateUser>, data: web::Data<Context>) -> impl Responder {
    let r = from_valid_error(req.validate());
    if r.is_err() {
        return Response::from_result(&r).to_json_resp();
    }

    let result = user::create(req.0, data).await;
    return Response::from_result(&result).to_json_resp();
}