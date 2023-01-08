use space10::Game;
use web_sys::HtmlCanvasElement;

use crate::{context::world_map::WorldMapContext, utils::html::get_canvas_context};

use super::WorldMapCanvasCache;

impl WorldMapCanvasCache {
    pub(super) fn draw_active_top_left(
        &self,
        game: &Game,
        world_map: &WorldMapContext,
        canvas: &HtmlCanvasElement,
    ) {
        let context = get_canvas_context(canvas);

        let selected_tile_id = world_map.selected_tile_id;
        let zone_id = game.to_zone_id(selected_tile_id);
        let (x, y) = game.zone_id_to_relative_coor(zone_id);
        let zone_length = Game::get_zone_length();
        let tile_scale_factor = world_map.tile_scale_factor;
        let scale_factor = (zone_length as u32 * tile_scale_factor) as f64;

        context.save();
        context.translate(0.5, 0.5).unwrap();
        context.scale(scale_factor, scale_factor).unwrap();
        context.set_line_width(2. / scale_factor);
        context.set_stroke_style(&"blue".into());

        let px = x;
        let py = y;
        context.begin_path();
        context.rect(px as f64, py as f64, 1., 1.);
        context.stroke();
        context.restore();
    }
}
