use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct Character {
    pub id: u16,
    pub name: String,
    pub actor_name: String,
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Character id: {}, name: {}, actor_name: {}",
            self.id, self.name, self.actor_name
        )
    }
}
