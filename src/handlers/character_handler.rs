use axum::{extract::Path, response::IntoResponse, Json};
use http::StatusCode;
use serde_json::{json, Value};

use crate::{
    models::Character,
    services::{
        character_service::{delete_new_character, update_new_character},
        create_new_character, get_all_characters, get_character_by_id,
    },
    utils::ApiResponse,
};

pub async fn get_characters() -> Result<ApiResponse, StatusCode> {
    match get_all_characters() {
        Ok(characters) => Ok(ApiResponse {
            status: StatusCode::OK,
            data: characters,
        }),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_characters_by_id(Path(id): Path<u16>) -> impl IntoResponse {
    match get_character_by_id(id) {
        Ok(Some(character)) => (StatusCode::OK, Json(character)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null)).into_response(),
    }
}

pub async fn create_character(Json(character): Json<Character>) -> impl IntoResponse {
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
