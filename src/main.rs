pub mod api;
pub mod app;
pub mod repository;
pub mod schema;
pub mod utils;

use crate::app::App;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = App::new().await;

    axum::serve(app.listener, app.router).await.unwrap()
}
