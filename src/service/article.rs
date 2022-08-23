use actix_web::web;
use sea_orm::DbErr;
use crate::Context;
use crate::cores::biz_code::{BizCode, server_error};
use crate::db::article;
use crate::models::request::create_article::CreateArticle;

pub async fn create(req: CreateArticle, data: web::Data<Context>) -> Result<u64, BizCode> {
    req.check()?;
    let r = article::insert(req.title, req.content, &data.conn).await;
    match r {
        Ok(d) => {Ok(d)}
        Err(e) => {Err(server_error())}
    }
}