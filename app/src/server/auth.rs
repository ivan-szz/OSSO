use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::sync::OnceLock;

static API_URL: OnceLock<String> = OnceLock::new();

fn get_api_url() -> &'static str {
    API_URL.get_or_init(|| env::var("API_URL").expect("Missing API_URL"))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[server]
pub async fn login_action(email: String, password: String) -> Result<LoginResponse, ServerFnError> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/api/auth/login", get_api_url()))
        .json(&json!({ "email": email, "password": password }))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Unable to login: {}", e)))?;

    if res.status().is_success() {
        Ok(res
            .json()
            .await
            .map_err(|e| ServerFnError::new(format!("Request failed: {}", e)))?)
    } else {
        Err(ServerFnError::new("Login failed."))
    }
}
