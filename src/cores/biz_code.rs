pub struct BizCode {
    pub code: i16,
    pub message: String,
}

pub const SUCCESS: i16 = 0;
pub const SERVER_ERROR: i16 = 500;
pub const CLIENT_ERROR: i16 = 400;
pub const DATA_NOTFOUND: i16 = 4040;

pub const TOKEN_INVALID: i16 = 10001;
pub const TOKEN_EXPIRE: i16 = 10002;
pub const REFRESH_TOKEN_INVALID: i16 = 10003;
pub const REFRESH_TOKEN_EXPIRE: i16 = 10004;

pub const USER_NAME_EXISTS: i16 = 20001;
pub const PHONE_INVALID: i16 = 20002;
pub const USER_NOT_EXISTS: i16 = 20003;
pub const PASSWORD_INVALID: i16 = 20004;

pub fn server_error() -> BizCode {
    return BizCode {
        code: SERVER_ERROR,
        message: "server_err".parse().unwrap(),
    };
}

pub fn ok() -> BizCode {
    return BizCode {
        code: SUCCESS,
        message: "ok".parse().unwrap(),
    };
}

pub fn biz(code: i16, message: &str) -> BizCode {
    BizCode{
        code,
        message: message.parse().unwrap()
    }
}
