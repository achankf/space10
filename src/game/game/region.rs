use std::ops::{Index, IndexMut};

use crate::{Game, Region, RegionId};

impl Index<RegionId> for Game {
    type Output = Region;

    fn index(&self, index: RegionId) -> &Self::Output {
        &self.regions[index]
    }
}

impl IndexMut<RegionId> for Game {
    fn index_mut(&mut self, index: RegionId) -> &mut Self::Output {
        &mut self.regions[index]
    }
}
