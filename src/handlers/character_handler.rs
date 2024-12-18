use axum::{extract::Path, response::IntoResponse, Json};
use http::StatusCode;

use crate::{
    services::{get_all_characters, get_character_by_id},
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
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::Value::Null),
        )
            .into_response(),
    }
}
