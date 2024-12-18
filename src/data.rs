use std::{collections::BTreeMap, sync::Mutex};

use once_cell::sync::Lazy;

use crate::models::Character;

fn initialize_data() -> Mutex<BTreeMap<u16, Character>> {
    Mutex::new(BTreeMap::from([
        (
            1,
            Character {
                id: 1,
                name: "Harry Potter".to_string(),
                actor_name: "Daniel RadCliff".to_string(),
            },
        ),
        (
            2,
            Character {
                id: 2,
                name: "Hermine Granger".to_string(),
                actor_name: "Emma Watson".to_string(),
            },
        ),
        (
            3,
            Character {
                id: 3,
                name: "Ron Wesley".to_string(),
                actor_name: "Rupert Grit".to_string(),
            },
        ),
    ]))
}

pub static DATA: Lazy<Mutex<BTreeMap<u16, Character>>> = Lazy::new(initialize_data);
