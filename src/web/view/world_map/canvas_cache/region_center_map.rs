use space10::{Game, ZoneId};

use web_sys::HtmlCanvasElement;

use crate::{context::world_map::WorldMapContext, utils::html::get_canvas_context};

use super::WorldMapCanvasCache;

impl WorldMapCanvasCache {
    pub(super) fn render_region_centers(
        &self,
        game: &Game,
        view_context: &WorldMapContext,
        canvas: &HtmlCanvasElement,
    ) {
        if !view_context.is_show_region_center {
            return;
        }

        let context = get_canvas_context(canvas);

        let num_regions = game.w_total_num_regions();
        let canvas_map_tile_length = view_context.tile_scale_factor;
        let zone_length = Game::get_zone_length();
        let scale_factor = (canvas_map_tile_length * zone_length as u32) as f64;

        let draw = |zone_id: ZoneId| {
            let (x, y) = game.zone_id_to_relative_coor(zone_id);

            context.rect((x) as f64, (y) as f64, 1., 1.);
        };

        context.save();
        context.translate(0.5, 0.5).unwrap();
        context.scale(scale_factor, scale_factor).unwrap();
        context.set_line_width(1. / scale_factor);
        context.set_stroke_style(&"purple".into());

        for region_id in 0..num_regions {
            let region_id = game.parse_region_id(region_id);
            let zone_id = game.get_region_center_zone_id(region_id);

            context.begin_path();
            draw(zone_id);
            context.stroke();
        }

        context.restore();
    }
}
