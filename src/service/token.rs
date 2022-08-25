use actix_web::web;
use crate::Context;
use crate::cores::biz_code::{biz, BizCode, from_code, PASSWORD_INVALID, SERVER_ERROR, USER_NOT_EXISTS};
use crate::cores::jwt::JWTToken;
use crate::db::user;
use crate::models::request::create_token::CreateToken;
use crate::cores::error::Result as Result2;


const SECRET: &str = "djduwlql";

pub fn validate_token(token: &str) -> Result2<JWTToken>{
    return JWTToken::verify(SECRET, token);
}

pub async fn create(req: CreateToken, data: web::Data<Context>) -> Result<String, BizCode>{
    let u = user::get_by_name(req.user_name, &data.conn).await?;
    if u.is_none() {
        return Err(from_code(USER_NOT_EXISTS))
    }

    if req.password.is_none() {
        return Err(from_code(PASSWORD_INVALID));
    }

    let u = u.unwrap();
    if !u.valid_password(&req.password.unwrap()) {
        return Err(from_code(PASSWORD_INVALID));
    }

    let jwt_token = JWTToken::new(&u.user_name.unwrap(), u.user_id as u64);
    let r = jwt_token.create_token(SECRET);
    if r.is_ok() {
        return Ok(r.unwrap());
    }

    return Err(biz(SERVER_ERROR, &r.err().unwrap().to_string()))
}