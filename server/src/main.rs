pub mod api;
pub mod repository;
pub mod schema;
pub mod utils;

use crate::api::routes::routes;
use app::*;
use axum::extract::FromRef;
use axum::Router;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub redis: redis::Client,
    pub leptos_options: LeptosOptions,
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("Missing env variable: DATABASE_URL");
    let redis_url = env::var("REDIS_URL").expect("Missing env variable: REDIS_URL");
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let letpos_routes_list = generate_route_list(App);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

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
        leptos_options: leptos_options.clone(),
    };

    let app = Router::new()
        .merge::<Router<AppState>>(routes())
        .leptos_routes(&state, letpos_routes_list, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    log!("listening on {}", &addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
