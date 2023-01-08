use std::collections::HashSet;

use stable_id_traits::CastUsize;

use crate::{
    RegionId, SuperRegionId, TileId, ZoneId,
    {AreaKind, Game, TileKind, REGION_LENGTH, SUPER_REGION_LENGTH, ZONE_LENGTH},
};

impl Game {
    const fn to_generic_id(&self, (x, y): (usize, usize), area_type: AreaKind) -> usize {
        let area_length = Self::area_type_to_length(area_type);
        let side_length = self.get_num_columns() / area_length;
        y * side_length + x
    }

    pub fn zone_coor_to_zone_id(&self, coor: (usize, usize)) -> ZoneId {
        CastUsize::cast_from(self.to_generic_id(coor, AreaKind::Zone))
    }

    pub fn region_coor_to_region_id(&self, coor: (usize, usize)) -> RegionId {
        CastUsize::cast_from(self.to_generic_id(coor, AreaKind::Region))
    }

    pub fn subdivide_square_areas(
        &self,
        id: usize,
        from: AreaKind,
        to: AreaKind,
    ) -> HashSet<usize> {
        let (from_len, to_len) = Self::area_type_pair_to_length_for_subdivision(from, to);

        Self::subdivide_square_areas_formula(id, self.get_num_columns(), from_len, to_len)
    }

    pub fn subdivide_square_areas_super_to_zone(&self, id: SuperRegionId) -> Vec<ZoneId> {
        self.subdivide_square_areas(id.cast_to(), AreaKind::Super, AreaKind::Zone)
            .into_iter()
            .map(CastUsize::cast_from)
            .collect()
    }

    pub fn subdivide_square_areas_super_to_region(&self, id: SuperRegionId) -> Vec<RegionId> {
        self.subdivide_square_areas(id.cast_to(), AreaKind::Super, AreaKind::Region)
            .into_iter()
            .map(CastUsize::cast_from)
            .collect()
    }

    pub fn subdivide_square_areas_zone_to_tile(&self, id: ZoneId) -> Vec<TileId> {
        self.subdivide_square_areas(id.cast_to(), AreaKind::Zone, AreaKind::Tile)
            .into_iter()
            .map(CastUsize::cast_from)
            .collect()
    }

    /**
     * Make sure the camera doesn't show invalid coordinates.
     * Notes: we're using unsigned integer so here we only need to make sure
     *        (x,y) doesn't go outside of the map's dimension.
     */
    pub const fn clamp_camera_max(
        &self,
        x: usize,
        y: usize,
        view_num_rows: usize,
        view_num_columns: usize,
    ) -> (usize, usize) {
        let x = if x + view_num_columns < self.num_columns {
            x
        } else {
            self.num_columns - view_num_columns
        };

        let y = if y + view_num_rows < self.num_rows {
            y
        } else {
            self.num_rows - view_num_rows
        };

        (x, y)
    }

    pub fn get_tile_type_from_zone(&self, zone_id: ZoneId) -> TileKind {
        self.zones[zone_id].tile_type
    }

    pub fn get_tile_type(&self, tile_id: TileId) -> TileKind {
        let zone_id = self.to_zone_id(tile_id);
        self.get_tile_type_from_zone(zone_id)
    }

    pub fn get_region_center_zone_id(&self, region_id: RegionId) -> ZoneId {
        self.regions[region_id].center_zone_id
    }

    pub fn get_all_land_regions(&self) -> Vec<RegionId> {
        self.land_super_region_vec
            .iter()
            .flat_map(|super_region_id| {
                self.subdivide_square_areas_super_to_region(*super_region_id)
            })
            .collect()
    }

    /**
     * Turn tile id (i.e. side_length = 1) into more general square area's id (scaled with input side_length).
     */
    fn generalize_tile_id(&self, tile_id: TileId, side_length: usize) -> usize {
        let (x, y) = self.to_coor(tile_id);
        let scaled_x = x / side_length;
        let scaled_y = y / side_length;
        let scaled_num_columns = self.num_columns / side_length;

        scaled_y * scaled_num_columns + scaled_x
    }

    pub fn to_zone_id(&self, tile_id: TileId) -> ZoneId {
        CastUsize::cast_from(self.generalize_tile_id(tile_id, ZONE_LENGTH))
    }

    pub fn to_region_id(&self, tile_id: TileId) -> RegionId {
        CastUsize::cast_from(self.generalize_tile_id(tile_id, REGION_LENGTH))
    }

    pub fn to_super_region_id(&self, tile_id: TileId) -> SuperRegionId {
        CastUsize::cast_from(self.generalize_tile_id(tile_id, SUPER_REGION_LENGTH))
    }

    pub fn to_zone_center_id(&self, zone_id: ZoneId) -> TileId {
        const HALF: usize = ZONE_LENGTH / 2;
        let (x, y) = self.zone_id_to_coor(zone_id);
        self.to_tile_id((x + HALF, y + HALF))
    }

    pub fn super_region_id_to_tile_id(&self, super_region_id: SuperRegionId) -> TileId {
        let (x, y) = self.super_region_id_to_coor(super_region_id);
        self.to_tile_id((x, y))
    }

    pub fn region_id_to_tile_id(&self, region_id: RegionId) -> TileId {
        let (x, y) = self.region_id_to_coor(region_id);
        self.to_tile_id((x, y))
    }

    pub fn zone_id_to_tile_id(&self, zone_id: ZoneId) -> TileId {
        let (x, y) = self.zone_id_to_coor(zone_id);
        self.to_tile_id((x, y))
    }

    pub fn zone_to_region_id(&self, zone_id: ZoneId) -> TileId {
        let (x, y) = self.zone_id_to_coor(zone_id);
        let side_length = REGION_LENGTH / ZONE_LENGTH;
        let scaled_x = x / side_length;
        let scaled_y = y / side_length;
        let scaled_num_columns = self.num_columns / side_length;

        TileId(scaled_y * scaled_num_columns + scaled_x)
    }

    pub fn get_map_frame(
        &self,
        top_left: TileId,
        num_rows: usize,
        num_columns: usize,
    ) -> Vec<TileId> {
        let area_size = num_rows * num_columns;

        let mut ret = Vec::with_capacity(area_size);

        let top_left_coor = self.to_coor(top_left);
        let (tx, ty) = top_left_coor;

        for y in 0..num_rows {
            let out_y = ty + y;
            for x in 0..num_columns {
                let out_x = tx + x;
                ret.push(self.to_tile_id((out_x, out_y)));
            }
        }

        ret
    }
}
