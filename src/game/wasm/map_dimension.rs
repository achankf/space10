use crate::{Game, RegionIdSizeType, SuperRegionIdSizeType, ZoneIdSizeType};

impl Game {
    pub fn w_surface_area(&self) -> usize {
        self.surface_area()
    }

    pub fn w_get_num_columns(&self) -> usize {
        self.get_num_columns()
    }

    pub fn w_get_num_rows(&self) -> usize {
        self.get_num_rows()
    }

    pub fn w_get_scaled_surface_area(&self, scale_length: usize) -> usize {
        self.get_scaled_surface_area(scale_length)
    }

    pub fn w_total_num_zones(&self) -> ZoneIdSizeType {
        self.total_num_zones()
    }

    pub fn w_num_zone_columns(&self) -> usize {
        self.num_zone_columns()
    }

    pub fn w_total_num_regions(&self) -> RegionIdSizeType {
        self.total_num_regions()
    }

    pub fn w_total_num_super_regions(&self) -> SuperRegionIdSizeType {
        self.total_num_super_regions()
    }

    pub fn w_total_num_land_regions(&self) -> RegionIdSizeType {
        self.total_num_land_regions()
    }
}
