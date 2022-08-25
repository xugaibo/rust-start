use serde::Deserialize;
use validator_derive::Validate;

#[derive(Deserialize, Validate)]
#[serde(default)]
pub struct CreateToken {
    #[validate(required, length(min = 3, max = 20, message = "user_name invalid"))]
    pub user_name: Option<String>,
    #[validate(required, length(min = 3, max = 20, message = "password invalid"))]
    pub password: Option<String>,
}

impl Default for CreateToken {
    fn default() -> Self {
        CreateToken {
            user_name: None,
            password: None,
        }
    }
}