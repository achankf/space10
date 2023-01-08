use stable_id_traits::Inner;

use crate::{CharacterIdSizeType, Game};

impl Game {
    pub fn w_get_player_id(&self) -> Option<CharacterIdSizeType> {
        self.player.map(Inner::project)
    }
}
