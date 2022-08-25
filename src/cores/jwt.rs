use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::cores::error::Error;
use crate::cores::error::Result;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTToken {
    id: String,
    username: String,
    exp: usize,
}

impl JWTToken {
    pub fn new(username: &str, id: u64) -> JWTToken {
        let now = SystemTime::now();
        // 30 分钟过期时间
        let m30 = Duration::from_secs(30 * 60);
        let now = now.duration_since(UNIX_EPOCH).expect("获取系统时间失败");
        JWTToken {
            id: id.to_string(),
            username: String::from(username),
            exp: (now + m30).as_secs() as usize,
        }
    }
    /// create token
    /// secret: your secret string
    pub fn create_token(&self, secret: &str) -> Result<String> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("JWTToken encode fail!")), // in practice you would return the error
        };
    }
    /// verify token invalid
    /// secret: your secret string
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken> {
        let mut validation = Validation::default();
        validation.validate_nbf = true;
        return match decode::<JWTToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => {
                let err_kind = err.kind();
                return match *err_kind {
                    ErrorKind::InvalidToken => Err(Error::from("InvalidToken")), // Example on how to handle a specific error
                    ErrorKind::InvalidIssuer => Err(Error::from("InvalidIssuer")), // Example on how to handle a specific error
                    ErrorKind::ExpiredSignature => Err(Error::from("ExpiredSignature")), // Example on how to handle a specific error
                    _ => Err(Error::from("InvalidToken other errors")),
                }
            }
        };
    }

    pub fn is_expire(&self) -> bool {
        return SystemTime::now().duration_since(UNIX_EPOCH).expect("获取系统时间失败").as_secs() as usize > self.exp;
    }
}

#[cfg(test)]
mod tests {
    use crate::cores::jwt::JWTToken;

    #[test]
    fn test_error() {
        let token = JWTToken::new("test", 123456).create_token("djduwlql");
        let r = JWTToken::verify("djduwlql", &token.ok().unwrap());
        if r.is_ok() {
            println!("valid ok")
        } else {
            println!("{}", r.err().unwrap().to_string());
        }


        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjEwIiwidXNlcm5hbWUiOiJ0ZXN0X3J1c3QiLCJleHAiOjE2NjE0MTk3MDJ9.HAIMjBRJTr0G0p4RdSXYbmxofpT0-lOdn2fDAOlPHA4";
        let r = JWTToken::verify("djduwlql", token);
        assert!(r.is_ok())
    }
}
