pub struct BizCode {
    pub code: String,
    pub message: String,
}

pub fn param_invalid() -> BizCode {
    return BizCode { code: "1".to_string(), message: "param invalid".parse().unwrap() };
}

pub fn ok() -> BizCode {
    return BizCode { code: "0".to_string(), message: "ok".parse().unwrap() };
}







