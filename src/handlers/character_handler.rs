use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use serde_json::{json, Value};
use sqlx::MySqlPool;

use crate::{
    models::Character,
    services::{
        character_service::{
            delete_new_character, get_character_by_id_from_db, update_new_character,
        },
        create_new_character, get_all_characters_from_db,
    },
    utils::ApiResponse,
};

pub async fn get_characters(State(db): State<MySqlPool>) -> impl IntoResponse {
    tracing::info!("Fetching all characters");

    match get_all_characters_from_db(State(db)).await {
        Ok(characters) => Ok(ApiResponse {
            status: StatusCode::OK,
            data: characters,
        }
        .into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// pub async fn get_characters_by_id(Path(id): Path<u16>) -> impl IntoResponse {
//     tracing::info!("Fetching single character with ID: {}", id);
//     match get_character_by_id(id) {
//         Ok(Some(character)) => (StatusCode::OK, Json(character)).into_response(),
//         Ok(None) => StatusCode::NOT_FOUND.into_response(),
//         Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null)).into_response(),
//     }
// }

pub async fn get_character_by_id(
    State(db): State<MySqlPool>,
    Path(id): Path<u16>,
) -> impl IntoResponse {
    tracing::info!("Fetching single character with ID: {}", id);
    match get_character_by_id_from_db(State(db), id).await {
        Ok(char) => (StatusCode::OK, Json(char)).into_response(),
        Err(StatusCode::NOT_FOUND) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null)).into_response(),
    }
}

pub async fn create_character(Json(character): Json<Character>) -> impl IntoResponse {
    tracing::info!("creating a character: {}", character);
    match create_new_character(character) {
        Ok(()) => (
            StatusCode::CREATED,
            Json(json!({
                "message": "Character created successfully"
            })),
        )
            .into_response(),
        Err(StatusCode::CONFLICT) => (
            StatusCode::CONFLICT,
            Json(json!({
                "message": "Character already exists"
            })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Internal server error"
            })),
        )
            .into_response(),
    }
}

pub async fn update_character(
    Path(id): Path<u16>,
    Json(updated_character): Json<Character>,
) -> impl IntoResponse {
    tracing::info!("updating a character with ID: {}", id);
    match update_new_character(id, updated_character) {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "message": "Character updated successfully"
            })),
        )
            .into_response(),
        Err(StatusCode::NOT_FOUND) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "message": "Character not found"
            })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Internal server error"
            })),
        )
            .into_response(),
    }
}

pub async fn delete_character(Path(id): Path<u16>) -> impl IntoResponse {
    tracing::info!("Deleting a character with ID: {}", id);
    match delete_new_character(id) {
        Ok(()) => (
            StatusCode::NO_CONTENT,
            Json(json!({
                "message": "Character deleted successfully"
            })),
        )
            .into_response(),
        Err(StatusCode::NOT_FOUND) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "message": "Character not found"
            })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Internal server error"
            })),
        )
            .into_response(),
    }
}
