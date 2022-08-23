use actix_web::HttpResponse;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::cores::biz_code::BizCode;

#[derive(Debug, Serialize, Clone)]
pub struct Response<T> {
    pub code: Option<i16>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> Response<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &Result<T, BizCode>) -> Self {
        if arg.is_ok() {
            let data = arg.as_ref().ok().unwrap();
            return Self {
                code: Some(0),
                msg: None,
                data: Some(data.clone()),
            };
        }

        let data = arg.as_ref().err().unwrap();
        return Self {
            code: Some(data.code),
            msg: Some(data.message.to_owned()),
            data: None,
        };
    }

    pub fn to_json_resp(&self) -> HttpResponse {
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
