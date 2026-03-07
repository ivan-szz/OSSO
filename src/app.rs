use crate::api::routes::routes;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
    redis: redis::Client,
}

pub struct App {
    pub router: Router,
    pub listener: TcpListener,
}

impl App {
    pub async fn new(db_url: String, redis_url: String, address: String) -> Self {
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

        let router = routes().with_state(state).layer(TraceLayer::new_for_http());
        let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
        println!("Server running on {}", address);

        Self { router, listener }
    }
}
