use std::ops::{Index, IndexMut};

use crate::{utils::js::log, Character, CharacterId, Family, FamilyId, Game, NationId, TileId};

impl Game {
    pub fn spawn_player(&mut self, first_name: String, family_name: String) -> CharacterId {
        assert!(self.player.is_none());

        let family_id = self.families.alloc(Family {
            name: family_name,
            ..Default::default()
        });

        let zone_id = self.get_random_settled_zone();
        let at = self.to_zone_center_id(zone_id);

        let character_id = self.characters.alloc(Character {
            name: first_name,
            family: family_id,
            at,
            birth: self.time,
            ..Default::default()
        });

        self.character_locations
            .entry(at)
            .or_default()
            .insert(character_id);

        self.player = Some(character_id);

        character_id
    }

    pub fn spawn_character(
        &mut self,
        name: String,
        at: TileId,
        family: FamilyId,
        faction_allegiance: Option<NationId>,
    ) -> CharacterId {
        let character_id = self.characters.alloc(Character {
            name,
            at,
            birth: self.time,
            family,
            faction_allegiance,
        });

        self.character_locations
            .entry(at)
            .or_insert_with(Default::default)
            .insert(character_id);

        character_id
    }

    pub fn get_player(&self) -> Option<&Character> {
        self.player.map(|id| &self.characters[id])
    }

    pub fn print_character(&self, id: CharacterId) {
        let c = serde_json::to_string_pretty(&self.characters[id]).unwrap();
        log(c);
    }
}

impl Index<CharacterId> for Game {
    type Output = Character;

    fn index(&self, index: CharacterId) -> &Self::Output {
        &self.characters[index]
    }
}

impl IndexMut<CharacterId> for Game {
    fn index_mut(&mut self, index: CharacterId) -> &mut Self::Output {
        &mut self.characters[index]
    }
}
