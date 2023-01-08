use crate::{
    Game, RegionId, SuperRegionId, TileId, ZoneId, REGION_LENGTH, SUPER_REGION_LENGTH, ZONE_LENGTH,
};

use rand::{seq::IteratorRandom, thread_rng, Rng};

impl Game {
    pub fn get_random_land_super_region_id(&self) -> SuperRegionId {
        *self
            .land_super_region_vec
            .iter()
            .choose(&mut thread_rng())
            .expect("cannot randomly pick a land tile")
    }

    pub fn get_random_land_region_id(&self) -> RegionId {
        let super_region_id = self.get_random_land_super_region_id();

        let (x, y) = self.project_subarea_starting_coor_super_to_region(super_region_id);

        let ratio = SUPER_REGION_LENGTH / REGION_LENGTH;

        let range = 0..ratio;

        let a = thread_rng().gen_range(range.clone());
        let b = thread_rng().gen_range(range);

        self.region_coor_to_region_id((x + a, y + b))
    }

    pub fn get_random_land_zone_center(&self) -> ZoneId {
        let region_id = self.get_random_land_region_id();
        self.regions[region_id].center_zone_id
    }

    pub fn get_random_land_tile(&self) -> TileId {
        let super_region_id = self.get_random_land_super_region_id();

        let (x, y) = self.super_region_id_to_coor(super_region_id);

        let a = thread_rng().gen_range(0..SUPER_REGION_LENGTH);
        let b = thread_rng().gen_range(0..SUPER_REGION_LENGTH);

        self.to_tile_id((x + a, y + b))
    }

    pub fn get_random_zone_id_in_region(&self, region_id: RegionId) -> ZoneId {
        let (x, y) = self.project_subarea_starting_coor_region_to_zone(region_id);

        let ratio = REGION_LENGTH / ZONE_LENGTH;

        // leave the outer tiles alone
        let range = 1..ratio - 1;

        let a = thread_rng().gen_range(range.clone());
        let b = thread_rng().gen_range(range);

        self.zone_coor_to_zone_id((x + a, y + b))
    }
}
