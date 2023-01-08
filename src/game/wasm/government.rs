use stable_id_traits::Inner;

use crate::{CharacterIdSizeType, Game, NationIdSizeType};

pub enum GovermentKind {
    Hermit,
    Autocracy,
    Republic,
}

impl Game {
    pub fn w_get_faction_head(&self, faction_id: NationIdSizeType) -> Option<CharacterIdSizeType> {
        let faction_id = self.parse_faction_id(faction_id);
        let faction = &self.nations[faction_id];

        faction.leader.map(Inner::project)
    }
}
