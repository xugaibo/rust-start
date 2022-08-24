use serde::Serialize;
#[derive(Debug, Clone, Serialize)]
pub enum Error {
    E(String),
    JsonError(String),
    RedisError(String),
    RbatisError(String),
    ValidationError(String),
}
pub type Result<T> = std::result::Result<T, Error>;

impl ToString for Error {
    fn to_string(&self) -> std::string::String {
        match self {
            Error::E(msg) => msg.clone(),
            Error::JsonError(msg) => msg.clone(),
            Error::RedisError(msg) => msg.clone(),
            Error::RbatisError(msg) => msg.clone(),
            Error::ValidationError(msg) => msg.clone(),
        }
        // serde_json::to_string(self).unwrap()
    }
}

// impl std::fmt::Display for Error {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
//         std::fmt::Debug::fmt(self, fmt)
//     }
// }

impl From<&str> for Error {
    fn from(msg: &str) -> Error {
        Error::E(msg.to_owned())
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Error {
        Error::E(msg)
    }
}

/// json error
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::JsonError(e.to_string())
    }
}


#[cfg(test)]
mod tests {
    use crate::cores::error::Error;

    #[test]
    fn test_error() {
        println!("{:?}", Error::from("错误的消息"));
    }
}
