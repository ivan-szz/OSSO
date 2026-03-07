pub mod api;
pub mod app;

use crate::app::App;
use std::env;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db_url = env::var("DATABASE_URL").expect("Missing env variable: DATABASE_URL");
    let redis_url = env::var("REDIS_URL").expect("Missing env variable: REDIS_URL");
    let address = String::from("0.0.0.0:8080");

    let app = App::new(db_url, redis_url, address).await;

    axum::serve(app.listener, app.router).await.unwrap()
}
