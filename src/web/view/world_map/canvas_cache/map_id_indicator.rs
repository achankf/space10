use space10::Game;
use web_sys::HtmlCanvasElement;

use crate::{
    context::world_map::WorldMapContext, enum_types::map_id_indicator_kind::MapIdIndicatorKind,
    utils::html::get_canvas_context,
};

use super::WorldMapCanvasCache;

impl WorldMapCanvasCache {
    pub(super) fn render_map_id_indicator(
        &self,
        game: &Game,
        view_context: &WorldMapContext,
        canvas: &HtmlCanvasElement,
    ) {
        if !view_context.is_show_map_id {
            return;
        }

        let context = get_canvas_context(canvas);
        let map_id_indicator_type = view_context.map_id_indicator_type;

        let map_view_rows = game.get_num_rows();
        let map_view_columns = game.get_num_columns();

        let (length, font_size) = match map_id_indicator_type {
            MapIdIndicatorKind::Zone => return,
            MapIdIndicatorKind::Region => (Game::get_region_length(), 0.58333377427827), // 7 pt
            MapIdIndicatorKind::SuperRegion => (Game::get_super_region_length(), 2.5),   // 30pt
        };

        let num_rows = map_view_rows / length;
        let num_columns = map_view_columns / length;
        let tile_scale_factor = view_context.tile_scale_factor;
        let scale_factor = (tile_scale_factor * length as u32) as f64;

        context.save();
        context.scale(scale_factor, scale_factor).unwrap();
        context.translate(0.5, 0.5).unwrap();
        context.set_font(&format!("{}rem Arial", font_size / length as f64));
        context.set_text_align("center");
        context.set_text_baseline("middle");
        context.set_fill_style(&"red".into());
        context.set_text_align("center");

        for y in 0..num_rows {
            for x in 0..num_columns {
                let px = x as f64;
                let py = y as f64;
                let id = y * num_columns + x;
                context
                    .fill_text(&format!("{id}"), px.floor(), py.floor())
                    .unwrap();
            }
        }

        context.restore();
    }
}
