use crate::cores::biz_code::{biz, BizCode};
use serde::Deserialize;
use crate::cores::biz_code::{CLIENT_ERROR, PASSWORD_INVALID};

#[derive(Deserialize)]
#[serde(default)]
pub struct CreateArticle {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl CreateArticle {
    pub fn check(&self) -> Result<i32, BizCode> {
        if self.title.is_none() {
            return Err(biz(CLIENT_ERROR, "title is null"))
        }
        if self.content.is_none() {
            return Err(biz(CLIENT_ERROR, "content is null"))
        }
        return Ok(1)
    }
}

impl Default for CreateArticle {
    fn default() -> Self {
        CreateArticle {
            title: None,
            content: None,
        }
    }
}
