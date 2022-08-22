use actix_web::HttpResponse;
use json::JsonValue;
use json::object::Object;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::cores::biz_code;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response<T> {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> Response<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &Result<T, String>) -> Self {
        match arg {
            Ok(data) => Self {
                code: Some("0".to_owned()),
                msg: None,
                data: Some(data.clone()),
            },
            Err(e) => {
                let p = json::parse(e);
                match p {
                    Ok(data) => {
                        Self {
                            code: Some(data["code"].take_string().unwrap()),
                            msg: Some(data["message"].take_string().unwrap()),
                            data: None,
                        }
                    }
                    Err(e) => Self{
                        code: Some("500".to_owned()),
                        msg: Some("server error".to_owned()),
                        data: None,
                    }
                }
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: Some("S".to_string()),
            msg: None,
            data: Some(arg.clone()),
        }
    }

    pub fn to_json_resp(&self) -> actix_http::Response {
        return HttpResponse::Ok()
            .content_type("json")
            .body(self.to_string());
    }
}

impl<T> ToString for Response<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
