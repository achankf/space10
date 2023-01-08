use crate::{Game, RegionIdSizeType, SuperRegionIdSizeType, TileIdSizeType, ZoneIdSizeType};

impl Game {
    pub fn w_to_coor(&self, tile_id: TileIdSizeType) -> Vec<usize> {
        let tile_id = self.parse_tile_id(tile_id);
        let (x, y) = self.to_coor(tile_id);
        vec![x, y]
    }

    pub fn w_super_region_id_to_coor(&self, super_region_id: SuperRegionIdSizeType) -> Vec<usize> {
        let super_region_id = self.parse_super_region_id(super_region_id);
        let (x, y) = self.super_region_id_to_coor(super_region_id);
        vec![x, y]
    }

    pub fn w_region_id_to_coor(&self, region_id: RegionIdSizeType) -> Vec<usize> {
        let region_id = self.parse_region_id(region_id);
        let (x, y) = self.region_id_to_coor(region_id);
        vec![x, y]
    }

    pub fn w_zone_id_to_coor(&self, zone_id: ZoneIdSizeType) -> Vec<usize> {
        let zone_id = self.parse_zone_id(zone_id);
        let (x, y) = self.zone_id_to_coor(zone_id);
        vec![x, y]
    }

    pub fn w_super_region_id_to_relative_coor(
        &self,
        super_region_id: SuperRegionIdSizeType,
    ) -> Vec<usize> {
        let super_region_id = self.parse_super_region_id(super_region_id);
        let (x, y) = self.super_region_id_to_relative_coor(super_region_id);
        vec![x, y]
    }

    pub fn w_region_id_to_relative_coor(&self, region_id: RegionIdSizeType) -> Vec<usize> {
        let region_id = self.parse_region_id(region_id);
        let (x, y) = self.region_id_to_relative_coor(region_id);
        vec![x, y]
    }

    pub fn w_zone_id_to_relative_coor(&self, zone_id: ZoneIdSizeType) -> Vec<usize> {
        let zone_id = self.parse_zone_id(zone_id);
        let (x, y) = self.zone_id_to_relative_coor(zone_id);
        vec![x, y]
    }

    pub fn w_to_tile_id(&self, x: usize, y: usize) -> usize {
        Self::from_generic_coor_formula((x, y), self.num_columns)
    }
}
