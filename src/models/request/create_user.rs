use serde::Deserialize;
use validator_derive::Validate;

#[derive(Deserialize, Validate)]
#[serde(default)]
pub struct CreateUser {
    #[validate(required, length(min = 3, max = 20, message = "user_name invalid"))]
    pub user_name: Option<String>,
    #[validate(required, length(min = 3, max = 20, message = "password invalid"))]
    pub password: Option<String>,
    #[validate(length(min = 11, max = 11, message = "user phone invalid"))]
    pub user_phone: Option<String>,
}

impl Default for CreateUser {
    fn default() -> Self {
        CreateUser {
            user_name: None,
            password: None,
            user_phone:None
        }
    }
}