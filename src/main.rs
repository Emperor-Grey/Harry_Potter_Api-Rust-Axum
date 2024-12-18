use std::net::SocketAddr;

use axum::{routing::get, Router};
use handlers::{get_characters, get_characters_by_id};
use tokio::net::TcpListener;

pub mod data;

pub mod handlers;
pub mod models;
pub mod services;
pub mod utils;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/characters", get(get_characters))
        .route("/characters/:id", get(get_characters_by_id));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
