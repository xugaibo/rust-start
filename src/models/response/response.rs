use actix_web::body::EitherBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpResponse};
use futures_util::future::LocalBoxFuture;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::cores::biz_code::{BizCode};

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

    pub(crate) fn from_biz_right<B>(biz_code: BizCode, req: ServiceRequest) -> LocalBoxFuture<'static, Result<ServiceResponse<EitherBody<B>>, Error>> where B: 'static {
        let (request, _pl) = req.into_parts();
        let result: Result<i16, BizCode> = Err(biz_code);
        let res = Response::from_result(&result).to_json_resp().map_into_right_body();
        return Box::pin(async { Ok(ServiceResponse::new(request, res)) });
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

