use std::panic;
use serde::__private::de::IdentifierDeserializer;

pub enum BizCode {
    Of(i16, String)
}

pub fn ParamInvalid() -> BizCode{
    return BizCode::Of(1, "param invalid".parse().unwrap());
}







