use rand::{seq::SliceRandom, thread_rng};

use crate::{Game, TileId};

impl Game {
    pub fn get_north(&self, tile_id: TileId) -> Option<TileId> {
        let (x, y) = self.to_coor(tile_id);

        if y == 0 {
            None
        } else {
            Some(self.to_tile_id((x, y - 1)))
        }
    }

    pub fn get_south(&self, tile_id: TileId) -> Option<TileId> {
        let (x, y) = self.to_coor(tile_id);

        let ny = y + 1;
        if ny >= self.num_rows {
            None
        } else {
            Some(self.to_tile_id((x, ny)))
        }
    }

    pub fn get_east(&self, tile_id: TileId) -> Option<TileId> {
        let (x, y) = self.to_coor(tile_id);

        let nx = x + 1;
        if nx >= self.num_columns {
            None
        } else {
            Some(self.to_tile_id((nx, y)))
        }
    }

    pub fn get_west(&self, tile_id: TileId) -> Option<TileId> {
        let (x, y) = self.to_coor(tile_id);

        if x == 0 {
            None
        } else {
            Some(self.to_tile_id((x - 1, y)))
        }
    }

    pub fn get_random_neighbour(&self, tile_id: TileId) -> Option<TileId> {
        let mut directions = [0, 1, 2, 3];
        directions.shuffle(&mut thread_rng());

        for d in directions {
            let coor = match d {
                0 => self.get_north(tile_id),
                1 => self.get_south(tile_id),
                2 => self.get_east(tile_id),
                3 => self.get_west(tile_id),
                _ => unreachable!("not a possibility"),
            };

            if coor.is_some() {
                return coor;
            }
        }

        // ... because each coordinate can have at most 2 None neighbours due to being in the corners
        unreachable!("cannot find random direction coor")
    }
}
