use serde::Deserialize;
use crate::cores::biz_error::param_invalid;

#[derive(Deserialize)]
#[serde(default)]
pub struct CreateArticle {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl CreateArticle {
    pub fn check(&self) -> String {
        if self.id.is_none() {
            panic!("{}", param_invalid());
        }
        return "ok".parse().unwrap();
    }
}

impl Default for CreateArticle {
    fn default() -> Self {
        CreateArticle { id: None, title: None, content: None }
    }
}





