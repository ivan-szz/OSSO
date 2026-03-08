use crate::api::auth;
use axum::Router;
use axum::routing::get;
use crate::AppState;

fn api_routes() -> Router<AppState> {
    Router::new().nest("/auth", auth::routes())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/healthcheck", get(root))
        .nest("/api", api_routes())
}

async fn root() -> &'static str {
    "OK"
}
