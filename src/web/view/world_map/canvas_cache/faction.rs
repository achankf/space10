use space10::{Game, ZoneId};

use web_sys::HtmlCanvasElement;

use crate::{context::world_map::WorldMapContext, utils::html::get_canvas_context};

use super::WorldMapCanvasCache;

impl WorldMapCanvasCache {
    pub(super) fn draw_faction(
        &self,
        game: &Game,
        view_context: &WorldMapContext,
        canvas: &HtmlCanvasElement,
    ) {
        if !view_context.is_show_region_center {
            return;
        }

        let context = get_canvas_context(canvas);
        let zone_length = Game::get_zone_length();
        let scale_factor = view_context.tile_scale_factor;
        let scale_factor = (scale_factor * zone_length as u32) as f64;

        let draw = |zone_id: ZoneId| {
            let (x, y) = game.zone_id_to_relative_coor(zone_id);

            context.rect((x) as f64, (y) as f64, 1., 1.);
        };

        context.save();
        context.translate(0.5, 0.5).unwrap();
        context.scale(scale_factor, scale_factor).unwrap();
        context.set_line_width(1. / scale_factor);

        let num_factions = game.w_get_factions_len();

        for faction_id in 0..num_factions {
            let color = game.w_get_faction_color(faction_id);
            context.set_fill_style(&color.clone().into());
            let capital_zone_id = game.w_get_capital_zone(faction_id);

            context.set_stroke_style(&color.into());

            for zone_id in game.w_get_faction_owned_zones(faction_id) {
                if zone_id != capital_zone_id {
                    let zone_id = game.parse_zone_id(zone_id);
                    context.begin_path();
                    draw(zone_id);
                    context.fill();
                    context.stroke();
                }
            }

            context.set_stroke_style(&"purple".into());
            context.begin_path();
            let capital_zone_id = game.parse_zone_id(capital_zone_id);
            draw(capital_zone_id);
            context.fill();
            context.stroke();
        }

        context.restore();
    }
}
