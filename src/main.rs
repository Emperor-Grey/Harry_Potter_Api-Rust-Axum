use std::net::SocketAddr;

use axum::{routing::get, Router};
use handlers::{
    create_character, delete_character, get_characters, get_characters_by_id, update_character,
};
use http::Method;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

pub mod data;

pub mod handlers;
pub mod models;
pub mod services;
pub mod utils;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET]),
        )
        .route("/characters", get(get_characters).post(create_character))
        .route(
            "/characters/:id",
            get(get_characters_by_id)
                .put(update_character)
                .delete(delete_character),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
