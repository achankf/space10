use crate::Game;
use stable_id_traits::Inner;

impl Game {
    pub fn w_get_north(&self, tile_id: usize) -> Option<usize> {
        let tile_id = self.parse_tile_id(tile_id);
        self.get_north(tile_id).map(Inner::project)
    }

    pub fn w_get_south(&self, tile_id: usize) -> Option<usize> {
        let tile_id = self.parse_tile_id(tile_id);
        self.get_south(tile_id).map(Inner::project)
    }

    pub fn w_get_east(&self, tile_id: usize) -> Option<usize> {
        let tile_id = self.parse_tile_id(tile_id);
        self.get_east(tile_id).map(Inner::project)
    }

    pub fn w_get_west(&self, tile_id: usize) -> Option<usize> {
        let tile_id = self.parse_tile_id(tile_id);
        self.get_west(tile_id).map(Inner::project)
    }

    pub fn w_get_random_neighbour(&self, tile_id: usize) -> Option<usize> {
        let tile_id = self.parse_tile_id(tile_id);
        self.get_random_neighbour(tile_id).map(Inner::project)
    }
}
