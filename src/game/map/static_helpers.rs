use std::collections::HashSet;

use crate::{AreaKind, Game, REGION_LENGTH, SUPER_REGION_LENGTH, ZONE_LENGTH};

impl Game {
    pub const fn to_generic_coor_formula(id: usize, side_length: usize) -> (usize, usize) {
        let y = id / side_length;
        let x = id % side_length;
        (x, y)
    }

    pub const fn from_generic_coor_formula((x, y): (usize, usize), side_length: usize) -> usize {
        y * side_length + x
    }

    pub const fn project_subarea_starting_coor_formula(
        id: usize,
        full_unit_length: usize, // defines the plane you do projections on
        from_num_columns: usize,
        to_num_columns: usize,
    ) -> (usize, usize) {
        let scaled_ratio = from_num_columns / to_num_columns;
        let scaled_from = full_unit_length / from_num_columns;
        let (sx, sy) = Self::to_generic_coor_formula(id, scaled_from);

        (sx * scaled_ratio, sy * scaled_ratio)
    }

    pub const fn area_type_to_length(area_type: AreaKind) -> usize {
        match area_type {
            AreaKind::Super => SUPER_REGION_LENGTH,
            AreaKind::Region => REGION_LENGTH,
            AreaKind::Zone => ZONE_LENGTH,
            AreaKind::Tile => 1,
        }
    }

    pub const fn area_type_pair_to_length_for_subdivision(
        from: AreaKind,
        to: AreaKind,
    ) -> (usize, usize) {
        let from_len = Self::area_type_to_length(from);
        let to_len = Self::area_type_to_length(to);

        assert!(
            from_len >= to_len,
            "need to subdivide from a larger area to smaller areas"
        );

        (from_len, to_len)
    }

    pub fn subdivide_square_areas_formula(
        id: usize,
        full_unit_length: usize, // defines the plane you do projections on
        from_num_columns: usize,
        to_num_columns: usize,
    ) -> HashSet<usize> {
        // project id to target space
        // calculate the dimension of a frame in the target space

        let scaled_ratio = from_num_columns / to_num_columns;
        let scaled_to = full_unit_length / to_num_columns;

        let (sx, sy) = Self::project_subarea_starting_coor_formula(
            id,
            full_unit_length,
            from_num_columns,
            to_num_columns,
        );

        let mut ret = HashSet::with_capacity(scaled_ratio * scaled_ratio);

        for y in 0..scaled_ratio {
            let dy = y + sy;
            for x in 0..scaled_ratio {
                let dx = x + sx;
                let id = Self::from_generic_coor_formula((dx, dy), scaled_to);
                ret.insert(id);
            }
        }

        ret
    }
}
