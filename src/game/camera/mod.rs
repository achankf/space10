use crate::{Game, TileId};

impl Game {
    pub fn project_center_to_camera_origin(
        &self,
        tile_id: TileId,
        view_num_rows: usize,
        view_num_columns: usize,
    ) -> TileId {
        let x_half = view_num_columns / 2;
        let y_half = view_num_rows / 2;

        let (x, y) = self.to_coor(tile_id);
        let x = if x < x_half { 0 } else { x - x_half };
        let y = if y < y_half { 0 } else { y - y_half };

        self.to_tile_id(self.clamp_camera_max(x, y, view_num_rows, view_num_columns))
    }
}
