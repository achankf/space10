use std::ops::{Index, IndexMut};

use crate::{Game, Zone, ZoneId};

impl Game {
    pub fn get_settled_zone(&self) -> impl Iterator<Item = (ZoneId, &Zone)> {
        self.zones
            .iter_with_id()
            .filter(|(_, zone)| zone.is_settled())
    }
}

impl Index<ZoneId> for Game {
    type Output = Zone;

    fn index(&self, index: ZoneId) -> &Self::Output {
        &self.zones[index]
    }
}

impl IndexMut<ZoneId> for Game {
    fn index_mut(&mut self, index: ZoneId) -> &mut Self::Output {
        &mut self.zones[index]
    }
}
