use crate::data::DATA;
use crate::models::character::Character;
use axum::extract::State;
use http::StatusCode;
use sqlx::MySqlPool;

// pub fn get_all_characters() -> Result<Vec<Character>, StatusCode> {
//     let data = match DATA.lock() {
//         Ok(data) => data,
//         Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
//     };

//     Ok(data.values().cloned().collect())
// }

pub async fn get_all_characters_from_db(
    State(db): State<MySqlPool>,
) -> Result<Vec<Character>, StatusCode> {
    let query = Character::select().build();

    let characters = sqlx::query_as::<_, Character>(&query).fetch_all(&db).await;

    match characters {
        Ok(data) => Ok(data),
        Err(err) => {
            tracing::error!("Database error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// pub fn get_character_by_id(id: u16) -> Result<Option<Character>, StatusCode> {
//     let data = match DATA.lock() {
//         Ok(data) => data,
//         Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
//     };

//     Ok(data.values().find(|&c| c.id == id).cloned())
// }

pub async fn get_character_by_id_from_db(
    State(db): State<MySqlPool>,
    id: u16,
) -> Result<Character, StatusCode> {
    let query = Character::select().where_id(id).build();

    let character = sqlx::query_as::<_, Character>(&query).fetch_one(&db).await;

    match character {
        Ok(data) => Ok(data),
        Err(err) => {
            tracing::error!("Database error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
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
