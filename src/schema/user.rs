use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UserSchema {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 32))]
    pub password: String,
}
