use axum::{routing::get, Router};
use dotenv::dotenv;
use handlers::{
    create_character, delete_all_characters, delete_character, get_character_by_id, get_characters,
    update_character,
};
use http::Method;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod dummy_data;

pub mod database;
pub mod handlers;
pub mod models;
pub mod services;
pub mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Database url issue");
    let pool = database::connect_database(&database_url).await;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Connected to database...");

    let app = Router::new()
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([
            Method::GET,
            Method::PUT,
            Method::POST,
            Method::DELETE,
        ]))
        .route(
            "/characters",
            get(get_characters)
                .post(create_character)
                .delete(delete_all_characters),
        )
        .route(
            "/characters/:id",
            get(get_character_by_id)
                .put(update_character)
                .delete(delete_character),
        )
        .with_state(pool);

    // Host and Port
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::debug!("listening on https://{}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
