use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;

use crate::models::Character;

#[derive(Debug, Serialize, Clone)]
pub struct ApiResponse {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub data: Vec<Character>,
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl ApiResponse {
    pub fn from_characters(characters: Vec<Character>) -> Self {
        ApiResponse {
            status: StatusCode::OK,
            data: characters,
        }
    }

    pub fn from_character(character: Character) -> Self {
        ApiResponse {
            status: StatusCode::OK,
            data: vec![character],
        }
    }
}
