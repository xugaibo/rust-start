use actix_web::web::Data;
use crate::Context;
use crate::cores::biz_code::BizCode;
use crate::db::user;
use crate::models::request::create_user::CreateUser;

pub(crate) async fn create(req: CreateUser, data: Data<Context>) -> Result<u64, BizCode> {
    let last_user_id = user::insert(req.user_name, req.user_phone, req.password, &data.conn).await?;
    return Ok(last_user_id);
}