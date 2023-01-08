use std::ops::{Index, IndexMut};

use crate::{Game, Tile, TileId};

impl Index<TileId> for Game {
    type Output = Tile;

    fn index(&self, index: TileId) -> &Self::Output {
        &self.tiles[index]
    }
}

impl IndexMut<TileId> for Game {
    fn index_mut(&mut self, index: TileId) -> &mut Self::Output {
        &mut self.tiles[index]
    }
}
