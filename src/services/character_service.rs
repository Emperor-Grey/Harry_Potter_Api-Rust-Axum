use crate::models::character::{Character, CharacterPayload};
use http::StatusCode;
use sqlx::MySqlPool;

// pub fn get_all_characters() -> Result<Vec<Character>, StatusCode> {
//     let data = match DATA.lock() {
//         Ok(data) => data,
//         Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
//     };

//     Ok(data.values().cloned().collect())
// }

pub async fn get_all_characters_from_db(db: MySqlPool) -> Result<Vec<Character>, StatusCode> {
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

pub async fn get_character_by_id_from_db(db: MySqlPool, id: u16) -> Result<Character, StatusCode> {
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

// pub fn create_new_character(character: Character) -> Result<(), StatusCode> {
//     let mut data = match DATA.lock() {
//         Ok(data) => data,
//         Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
//     };

//     if data.contains_key(&character.id) {
//         return Err(StatusCode::CONFLICT);
//     }

//     data.insert(character.id, character);
//     Ok(())
// }

pub async fn create_new_character_in_db(
    db: MySqlPool,
    char: CharacterPayload,
) -> Result<StatusCode, StatusCode> {
    let query = Character::insert()
        .insert_to_actor_name(char.actor_name)
        .insert_to_name(char.name)
        .build();

    match sqlx::query(&query).execute(&db).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(err) => {
            tracing::error!("Database error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// pub fn update_new_character(id: u16, updated_character: Character) -> Result<(), StatusCode> {
//     let mut data = match DATA.lock() {
//         Ok(data) => data,
//         Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
//     };

//     if let Some(existing_character) = data.get_mut(&id) {
//         *existing_character = updated_character;
//         Ok(())
//     } else {
//         Err(StatusCode::NOT_FOUND)
//     }
// }

pub async fn update_new_character_in_db(
    db: MySqlPool,
    update_character: CharacterPayload,
    id: u16,
) -> Result<StatusCode, StatusCode> {
    let query = Character::update()
        .update_actor_name_with_value(update_character.actor_name)
        .update_name_with_value(update_character.name)
        .update_where_id_eq(id);

    let updated_char = sqlx::query(&query).execute(&db).await;

    match updated_char {
        Ok(_) => {
            tracing::info!("Character with ID {} updated successfully", id);
            Ok(StatusCode::OK)
        }

        Err(err) => {
            tracing::error!("Database error during update: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// pub fn delete_new_character(id: u16) -> Result<(), StatusCode> {
//     let mut data = match DATA.lock() {
//         Ok(data) => data,
//         Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
//     };

//     if data.remove(&id).is_some() {
//         Ok(())
//     } else {
//         Err(StatusCode::NOT_FOUND)
//     }
// }

pub async fn delete_new_character_from_db(
    db: MySqlPool,
    id: u16,
) -> Result<StatusCode, StatusCode> {
    let query = Character::delete().delete_where_id_eq(id);

    match sqlx::query(&query).execute(&db).await {
        Ok(_d) => {
            tracing::info!("Character with ID {} successfully deleted", id);
            Ok(StatusCode::OK)
        }

        Err(err) => {
            tracing::error!("Database error during update: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_all_character_from_db(db: MySqlPool) -> Result<StatusCode, StatusCode> {
    let query = "DELETE FROM characters";

    match sqlx::query(query).execute(&db).await {
        Ok(_d) => Ok(StatusCode::OK),
        Err(err) => {
            tracing::error!("Database error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
