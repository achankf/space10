use stable_id_traits::Inner;

use crate::{CharacterId, CharacterIdSizeType, Game, NationIdSizeType, TileId};

impl Game {
    pub fn get_characters_len(&self) -> usize {
        self.characters.len()
    }

    pub fn get_character_first_name(&self, id: CharacterId) -> String {
        self.characters[id].name.clone()
    }

    pub fn get_character_family_name(&self, id: CharacterId) -> String {
        let family_id = self.characters[id].family;

        self.get_family_name(family_id)
    }

    pub fn get_character_at(&self, id: CharacterId) -> TileId {
        self.characters[id].at
    }

    pub fn get_character_faction_allegiance(
        &self,
        id: CharacterIdSizeType,
    ) -> Option<NationIdSizeType> {
        self.parse_character_id(id)
            .and_then(|id| self.characters[id].faction_allegiance.map(Inner::project))
    }
}
