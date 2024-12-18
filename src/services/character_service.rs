use http::StatusCode;

use crate::data::DATA;
use crate::models::character::Character;

pub fn get_all_characters() -> Result<Vec<Character>, StatusCode> {
    let data = match DATA.lock() {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(data.values().cloned().collect())
}

pub fn get_character_by_id(id: u16) -> Result<Option<Character>, StatusCode> {
    let data = match DATA.lock() {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(data.values().cloned().find(|c| c.id == id))
}
