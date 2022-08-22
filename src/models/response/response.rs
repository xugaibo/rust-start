use actix_web::HttpResponse;
use serde::de::DeserializeOwned;
use serde::Serialize;


#[derive(Debug, Serialize, Clone)]
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
        if arg.is_ok() {
            let data = arg.as_ref().ok().unwrap();
            return Self {
                code: Some("0".to_owned()),
                msg: None,
                data: Some(data.clone()),
            }
        }

        let data = arg.as_ref().err().unwrap();
        let r = json::parse(data.as_str());
        if r.is_ok() {
            let json = r.as_ref().ok().unwrap();
            let code = json["code"].to_string();
            let message = json["msg"].to_string();
            return Self {
                code: Some(code),
                msg: Some(message),
                data: None,
            }
        }

        return Self{
            code: Some("500".to_owned()),
            msg: Some("server error".to_owned()),
            data: None,
        }
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
