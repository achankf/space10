use crate::{
    BuildingKind, Game, RegionIdSizeType, SuperRegionIdSizeType, TileIdSizeType, TileKind,
    ZoneIdSizeType,
};
use stable_id_traits::Inner;

pub enum OwnershipKind {
    Private,
    Public,
    Organization,
    Equity,
    None,
}

impl Game {
    pub fn w_get_tile_building_type(&self, tile_id: TileIdSizeType) -> Option<BuildingKind> {
        let tile_id = self.parse_tile_id(tile_id);
        self.tiles[tile_id].building.map(|(building, _)| building)
    }

    pub fn w_to_zone_id(&self, tile_id: TileIdSizeType) -> ZoneIdSizeType {
        let tile_id = self.parse_tile_id(tile_id);
        self.to_zone_id(tile_id).project()
    }

    pub fn w_to_region_id(&self, tile_id: TileIdSizeType) -> RegionIdSizeType {
        let tile_id = self.parse_tile_id(tile_id);
        self.to_region_id(tile_id).project()
    }

    pub fn w_to_super_region_id(&self, tile_id: TileIdSizeType) -> SuperRegionIdSizeType {
        let tile_id = self.parse_tile_id(tile_id);
        self.to_super_region_id(tile_id).project()
    }

    pub fn w_to_zone_center_id(&self, zone_id: ZoneIdSizeType) -> TileIdSizeType {
        let zone_id = self.parse_zone_id(zone_id);
        self.to_zone_center_id(zone_id).project()
    }

    pub fn w_get_tile_type_from_zone(&self, zone_id: ZoneIdSizeType) -> TileKind {
        let zone_id = self.parse_zone_id(zone_id);
        self.get_tile_type_from_zone(zone_id)
    }

    pub fn w_get_tile_type(&self, tile_id: TileIdSizeType) -> TileKind {
        let tile_id = self.parse_tile_id(tile_id);
        let zone_id = self.to_zone_id(tile_id);
        self.get_tile_type_from_zone(zone_id)
    }

    pub fn w_get_map_frame(
        &self,
        top_left: TileIdSizeType,
        num_rows: usize,
        num_columns: usize,
    ) -> Vec<usize> {
        let top_left = self.parse_tile_id(top_left);
        self.get_map_frame(top_left, num_rows, num_columns)
            .into_iter()
            .map(|tile_id| tile_id.0)
            .collect()
    }
}
