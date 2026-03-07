use crate::api::auth::{login, register};
use crate::app::AppState;
use axum::Router;
use axum::routing::{get, post};

pub fn routes<T>() -> Router<AppState>
where
    T: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(root))
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn root() -> &'static str {
    "OK"
}
