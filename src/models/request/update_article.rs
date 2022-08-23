use serde::Deserialize;
use crate::cores::biz_code::{biz, BizCode, CLIENT_ERROR};

#[derive(Deserialize)]
#[serde(default)]
pub struct UpdateArticle {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl UpdateArticle {
    pub fn check(&self) -> Result<i32, BizCode>{
        if self.id.is_none() {
            return Err(biz(CLIENT_ERROR, "id is null"));
        }
        return Ok(1);
    }
}

impl Default for UpdateArticle {
    fn default() -> Self {
        UpdateArticle {
            id: None,
            title: None,
            content: None,
        }
    }
}
