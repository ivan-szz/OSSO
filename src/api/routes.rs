use crate::api::auth;
use crate::app::AppState;
use axum::Router;
use axum::routing::get;

fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::routes())
}

pub fn routes<T>() -> Router<AppState>
where
    T: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(root))
        .nest("/api", api_routes())
}

async fn root() -> &'static str {
    "OK"
}
