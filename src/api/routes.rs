use crate::api::auth;
use crate::app::AppState;
use axum::Router;
use axum::routing::get;

pub fn routes<T>() -> Router<AppState>
where
    T: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(root))
        .nest("/auth", auth::routes())
}

async fn root() -> &'static str {
    "OK"
}
