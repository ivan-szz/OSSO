use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RefreshTokenSchema {
    #[validate(length(min = 1, max = 1024))]
    pub refresh_token: String,
}
