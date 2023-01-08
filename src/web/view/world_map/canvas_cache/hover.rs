use space10::{Game, TileId};
use stable_id_traits::CastUsize;
use web_sys::HtmlCanvasElement;

use crate::{
    context::world_map::WorldMapContext, enum_types::map_id_indicator_kind::MapIdIndicatorKind,
    utils::html::get_canvas_context,
};

use super::WorldMapCanvasCache;

impl WorldMapCanvasCache {
    pub(super) fn draw_hover(
        &self,
        game: &Game,
        view_context: &WorldMapContext,
        canvas: &HtmlCanvasElement,
        map_id_indicator_type: MapIdIndicatorKind,
        world_map_hover_tile_id: TileId,
    ) {
        // draw the zone-level cursor regardless of the exact type
        if map_id_indicator_type != MapIdIndicatorKind::Zone {
            self.draw_hover(
                game,
                view_context,
                canvas,
                MapIdIndicatorKind::Zone,
                world_map_hover_tile_id,
            );
        }

        let (length, id) = match map_id_indicator_type {
            MapIdIndicatorKind::Zone => (
                Game::get_zone_length(),
                game.to_zone_id(world_map_hover_tile_id).cast_to(),
            ),
            MapIdIndicatorKind::Region => (
                Game::get_region_length() as usize,
                game.to_region_id(world_map_hover_tile_id).cast_to(),
            ),
            MapIdIndicatorKind::SuperRegion => (
                Game::get_super_region_length() as usize,
                game.to_super_region_id(world_map_hover_tile_id).cast_to(),
            ),
        };

        let map_view_columns = game.get_num_columns();
        let num_columns = map_view_columns / length;
        let x = id % num_columns;
        let y = id / num_columns;
        let canvas_map_tile_length = view_context.tile_scale_factor;
        let scale_factor = (length as u32 * canvas_map_tile_length) as f64;

        let context = get_canvas_context(canvas);
        context.save();
        context.translate(0.5, 0.5).unwrap();
        context.scale(scale_factor, scale_factor).unwrap();
        context.set_line_width(2. / scale_factor);
        context.set_stroke_style(&"purple".into());

        context.begin_path();
        context.rect(x as f64, y as f64, 1., 1.);
        context.stroke();
        context.restore();
    }
}
