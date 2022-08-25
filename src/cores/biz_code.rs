use phf::{phf_map};
use validator::{ValidationErrors};

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

// const map = HashMap::new();

static KV: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "success",
    "500" => "server error",
    "400" => "client error",
    "4040" => "data not dound",
    "10001" => "token invalid",
    "10002" => "token expire",
    "10003" => "refresh token invalid",
    "10004" => "refresh token expire",
    "20001" => "user name exists",
    "20002" => "phone invalid",
    "20003" => "user not exists",
    "20004" => "password invalid",
};

pub fn from_code(code: i16) -> BizCode {
    let message = KV.get(code.to_string().as_ref());
    if message.is_none() {
        return server_error();
    }

    return BizCode {
        code,
        message: message.unwrap().to_string(),
    };
}
pub fn server_error() -> BizCode {
    return BizCode {
        code: SERVER_ERROR,
        message: "server_err".parse().unwrap(),
    };
}

pub fn token_invalid() -> BizCode {
    return BizCode {
        code: TOKEN_INVALID,
        message: "token_invalid".parse().unwrap(),
    };
}

pub fn ok() -> BizCode {
    return BizCode {
        code: SUCCESS,
        message: "ok".parse().unwrap(),
    };
}

pub fn biz(code: i16, message: &str) -> BizCode {
    BizCode {
        code,
        message: message.parse().unwrap(),
    }
}

pub fn from_valid_error(r: Result<(), ValidationErrors>) -> Result<(), BizCode> {
    if r.is_ok() {
        return Ok(());
    }
    let error = r.err().unwrap().to_string();
    let error = biz(CLIENT_ERROR,&error);
    return Err(error);
}


