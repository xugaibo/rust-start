use crate::cores::biz_code;
use crate::models::response::response;

fn biz_error(biz_code: biz_code::BizCode) -> String {
    let res: response::Response<String> = response::Response{
        code: Some(biz_code.code),
        msg: Some(biz_code.message),
        data: None,
    };
    return res.to_string();
}

pub fn param_invalid() -> String {
    return biz_error(biz_code::param_invalid());
}


