use space10::Game;

use web_sys::HtmlCanvasElement;

use crate::{context::world_map::WorldMapContext, utils::html::get_canvas_context};

use super::WorldMapCanvasCache;

impl WorldMapCanvasCache {
    pub(super) fn render_terrain_map(
        &self,
        game: &Game,
        view_context: &WorldMapContext,
        canvas: &HtmlCanvasElement,
    ) {
        let context = get_canvas_context(canvas);
        let zone_length = Game::get_zone_length();
        let canvas_map_tile_length = view_context.tile_scale_factor;
        let scale_factor = (canvas_map_tile_length * zone_length as u32) as f64;

        context.save();

        context
            .scale(scale_factor.floor(), scale_factor.floor())
            .unwrap();

        context.set_fill_style(&"white".into());
        context.fill_rect(0., 0., canvas.width() as f64, canvas.height() as f64);

        let max_viewport_columns =
            (view_context.viewport_width as f64 / scale_factor).ceil() as usize;
        let max_viewport_rows =
            (view_context.viewport_height as f64 / scale_factor).ceil() as usize;

        let x_start = 0;
        let x_end = (x_start + max_viewport_columns).min(game.num_zone_columns());
        let y_start = 0;
        let y_end = (y_start + max_viewport_rows).min(game.num_zone_rows());

        for x in x_start..x_end {
            for y in y_start..y_end {
                let zone_id = game.zone_coor_to_zone_id((x, y));
                let (zx, zy) = game.zone_id_to_relative_coor(zone_id);
                let tile_type = game.get_tile_type_from_zone(zone_id);

                context.set_fill_style(&tile_type.get_color().into());
                context.fill_rect(zx as f64, zy as f64, 1., 1.);
            }
        }

        context.restore();
    }
}
