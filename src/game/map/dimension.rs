use std::convert::TryInto;

use stable_id_traits::CastUsize;

use crate::{
    Game, RegionIdSizeType, SuperRegionIdSizeType, TileIdSizeType, ZoneIdSizeType, REGION_LENGTH,
    SUPER_REGION_LENGTH, ZONE_LENGTH,
};

impl Game {
    pub const fn surface_area(&self) -> TileIdSizeType {
        self.num_columns * self.num_rows
    }

    pub const fn get_num_columns(&self) -> TileIdSizeType {
        self.num_columns
    }

    pub const fn get_num_rows(&self) -> TileIdSizeType {
        self.num_rows
    }

    pub const fn get_num_region_rows(&self) -> RegionIdSizeType {
        (self.get_num_rows() as usize / Self::get_region_length()) as RegionIdSizeType
    }

    pub const fn get_num_region_columns(&self) -> RegionIdSizeType {
        (self.get_num_columns() as usize / Self::get_region_length()) as RegionIdSizeType
    }

    pub const fn get_num_super_region_rows(&self) -> SuperRegionIdSizeType {
        (self.get_num_rows() as usize / Self::get_super_region_length()) as SuperRegionIdSizeType
    }

    pub const fn get_num_super_region_columns(&self) -> SuperRegionIdSizeType {
        (self.get_num_columns() as usize / Self::get_super_region_length()) as SuperRegionIdSizeType
    }

    // note: squared scale
    pub fn get_scaled_surface_area(&self, scale_length: usize) -> usize {
        CastUsize::cast_to(self.surface_area()) / (scale_length * scale_length)
    }

    pub fn total_num_zones(&self) -> ZoneIdSizeType {
        self.get_scaled_surface_area(ZONE_LENGTH)
            .try_into()
            .unwrap()
    }

    pub fn num_zone_columns(&self) -> usize {
        self.num_columns / ZONE_LENGTH
    }

    pub fn num_zone_rows(&self) -> usize {
        self.num_rows / ZONE_LENGTH
    }

    pub fn total_num_regions(&self) -> RegionIdSizeType {
        self.get_scaled_surface_area(REGION_LENGTH)
            .try_into()
            .unwrap()
    }

    pub fn total_num_super_regions(&self) -> SuperRegionIdSizeType {
        self.get_scaled_surface_area(SUPER_REGION_LENGTH)
            .try_into()
            .unwrap()
    }

    pub fn total_num_land_regions(&self) -> RegionIdSizeType {
        const SIDE_LENGTH: usize = SUPER_REGION_LENGTH / REGION_LENGTH;
        const SIDE_LENGTH_SQUARED: usize = SIDE_LENGTH * SIDE_LENGTH;
        (self.land_super_region_vec.len() * SIDE_LENGTH_SQUARED)
            .try_into()
            .unwrap()
    }
}
