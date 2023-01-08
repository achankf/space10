use stable_id_traits::{CastUsize, Inner};

use crate::{AreaKind, Game, RegionId, SuperRegionId, TileId, ZoneId, ZONE_LENGTH};

impl Game {
    pub fn to_coor(&self, tile_id: TileId) -> (usize, usize) {
        Self::to_generic_coor_formula(tile_id.project(), self.num_columns)
    }

    pub const fn to_tile_id(&self, coor: (usize, usize)) -> TileId {
        TileId(Self::from_generic_coor_formula(coor, self.num_columns))
    }

    pub fn super_region_id_to_coor(&self, super_region_id: SuperRegionId) -> (usize, usize) {
        self.project_subarea_starting_coor(
            CastUsize::cast_to(super_region_id),
            AreaKind::Super,
            AreaKind::Tile,
        )
    }

    pub fn region_id_to_coor(&self, region_id: RegionId) -> (usize, usize) {
        self.project_subarea_starting_coor(
            CastUsize::cast_to(region_id),
            AreaKind::Region,
            AreaKind::Tile,
        )
    }

    pub fn zone_id_to_coor(&self, zone_id: ZoneId) -> (usize, usize) {
        self.project_subarea_starting_coor(
            CastUsize::cast_to(zone_id),
            AreaKind::Zone,
            AreaKind::Tile,
        )
    }

    pub fn super_region_id_to_relative_coor(
        &self,
        super_region_id: SuperRegionId,
    ) -> (usize, usize) {
        self.project_subarea_starting_coor(
            CastUsize::cast_to(super_region_id),
            AreaKind::Super,
            AreaKind::Super,
        )
    }

    pub fn region_id_to_relative_coor(&self, region_id: RegionId) -> (usize, usize) {
        self.project_subarea_starting_coor(
            CastUsize::cast_to(region_id),
            AreaKind::Region,
            AreaKind::Region,
        )
    }

    pub fn zone_id_to_relative_coor(&self, zone_id: ZoneId) -> (usize, usize) {
        self.project_subarea_starting_coor(
            CastUsize::cast_to(zone_id),
            AreaKind::Zone,
            AreaKind::Zone,
        )
    }

    pub const fn project_subarea_starting_coor(
        &self,
        id: usize,
        from: AreaKind,
        to: AreaKind,
    ) -> (usize, usize) {
        let (from_len, to_len) = Self::area_type_pair_to_length_for_subdivision(from, to);

        Self::project_subarea_starting_coor_formula(id, self.get_num_columns(), from_len, to_len)
    }

    pub fn project_subarea_starting_coor_super_to_region(
        &self,
        super_region_id: SuperRegionId,
    ) -> (usize, usize) {
        self.project_subarea_starting_coor(
            super_region_id.cast_to(),
            AreaKind::Super,
            AreaKind::Region,
        )
    }

    pub fn project_subarea_starting_coor_region_to_zone(
        &self,
        region_id: RegionId,
    ) -> (usize, usize) {
        self.project_subarea_starting_coor(region_id.cast_to(), AreaKind::Region, AreaKind::Zone)
    }

    /*
    pub const fn check_zone_sub_center(&self, tile_id: TileId) -> bool {
        let (x, y) = self.to_coor(tile_id);
        (y % 5 == 3 || y % 5 == 1) && (x % 5 == 3 || x % 5 == 1)
    }
    */

    pub fn check_zone_center(&self, tile_id: TileId) -> bool {
        const HALF: usize = ZONE_LENGTH / 2;
        let (x, y) = self.to_coor(tile_id);
        x % ZONE_LENGTH == HALF && y % ZONE_LENGTH == HALF
    }

    pub fn check_region_center(&self, tile_id: TileId) -> bool {
        let zone_id = self.to_zone_id(tile_id);
        self.zones[zone_id].is_region_center
    }
}
