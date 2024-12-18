use crate::data::DATA;
use crate::models::character::Character;
use http::StatusCode;

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

    Ok(data.values().find(|&c| c.id == id).cloned())
}

pub fn create_new_character(character: Character) -> Result<(), StatusCode> {
    let mut data = match DATA.lock() {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if data.contains_key(&character.id) {
        return Err(StatusCode::CONFLICT);
    }

    data.insert(character.id, character);
    Ok(())
}

pub fn update_new_character(id: u16, updated_character: Character) -> Result<(), StatusCode> {
    let mut data = match DATA.lock() {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if let Some(existing_character) = data.get_mut(&id) {
        *existing_character = updated_character;
        Ok(())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub fn delete_new_character(id: u16) -> Result<(), StatusCode> {
    let mut data = match DATA.lock() {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if data.remove(&id).is_some() {
        Ok(())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
