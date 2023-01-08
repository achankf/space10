use stable_id::Tec;

use crate::{utils::js::log, Game, REGION_LENGTH, SUPER_REGION_LENGTH, ZONE_LENGTH};

impl Game {
    pub fn allocate(row_scale: usize, column_scale: usize) -> Self {
        assert!(
            row_scale * column_scale >= 10,
            "can't allocate world with less than 10 super regions"
        );

        let num_rows = row_scale * SUPER_REGION_LENGTH;
        let num_columns = column_scale * SUPER_REGION_LENGTH;

        let surface_area = num_rows * num_columns;
        let zone_surface_area = surface_area / ZONE_LENGTH;
        let region_surface_area = surface_area / REGION_LENGTH;

        log(format!(
            r#"allocating world map: rows={num_rows} columns={num_columns}
surface_areas: tile={surface_area}, zone={zone_surface_area}, region={region_surface_area}
"#,
        ));

        let tiles = Tec::populate_defaults(surface_area);
        let zones = Tec::populate_defaults(zone_surface_area);
        let regions = Tec::populate_defaults(region_surface_area);

        Self {
            num_columns,
            num_rows,
            tiles,
            zones,
            regions,
            ..Default::default()
        }
    }
}
