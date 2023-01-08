use space10::Game;

use web_sys::HtmlCanvasElement;

use crate::{context::world_map::WorldMapContext, utils::html::get_canvas_context};

use super::WorldMapCanvasCache;

impl WorldMapCanvasCache {
    pub(super) fn render_super_region_grid(
        &self,
        game: &Game,
        view_context: &WorldMapContext,
        canvas: &HtmlCanvasElement,
    ) {
        let context = get_canvas_context(canvas);

        let num_super_region_rows = game.get_num_super_region_rows() as usize;
        let num_superregion_columns = game.get_num_super_region_columns() as usize;
        let super_region_length = Game::get_super_region_length();
        let canvas_map_tile_length = view_context.tile_scale_factor;
        let scale_factor = (canvas_map_tile_length * super_region_length as u32) as f64;

        context.save();
        context.translate(0.5, 0.5).unwrap();
        context.scale(scale_factor, scale_factor).unwrap();
        context.set_line_width(1. / scale_factor);

        for i in 0..=(num_superregion_columns as usize) {
            context.begin_path();
            let x = (i) as f64;
            let y = (num_super_region_rows) as f64;
            context.move_to(x, 0.);
            context.line_to(x, y);
            context.stroke();
        }

        for i in 0..=(num_superregion_columns as usize) {
            context.begin_path();
            let y = (i) as f64;
            let x = (num_superregion_columns) as f64;
            context.move_to(0., y);
            context.line_to(x, y);
            context.stroke();
        }

        context.restore();
    }
}
