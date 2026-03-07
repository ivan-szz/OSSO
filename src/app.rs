use crate::api::routes::routes;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub redis: redis::Client,
}

pub struct App {
    pub router: Router,
    pub listener: TcpListener,
}

impl App {
    pub async fn new() -> Self {
        let db_url = env::var("DATABASE_URL").expect("Missing env variable: DATABASE_URL");
        let redis_url = env::var("REDIS_URL").expect("Missing env variable: REDIS_URL");
        let address = String::from("0.0.0.0:8080");

        let pool = PgPoolOptions::new()
            .connect(&db_url)
            .await
            .expect("Unable to connect to database.");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Error while running migrations");

        let redis_client = redis::Client::open(redis_url).expect("Unable to connect to redis");
        redis_client
            .get_connection()
            .expect("Unable to get redis connection");

        let state = AppState {
            db: pool,
            redis: redis_client,
        };

        let router = routes::<AppState>()
            .with_state(state)
            .layer(TraceLayer::new_for_http());
        let listener = TcpListener::bind(&address).await.unwrap();
        println!("Server running on {}", address);

        Self { router, listener }
    }
}
