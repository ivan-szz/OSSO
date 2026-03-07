use axum::Router;
use axum::routing::get;

pub fn routes<T>() -> Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    Router::new().route("/", get(root))
}

async fn root() -> &'static str {
    "OK"
}
