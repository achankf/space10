mod active_map_top_left;
mod faction;
mod hover;
mod map_id_indicator;
mod region_center_map;
mod region_grid;
mod super_region_grid;
mod terrain_map;

use space10::Game;
use web_sys::HtmlCanvasElement;

use crate::{
    context::world_map::WorldMapContext,
    utils::html::{create_canvas, get_canvas_context},
};

#[derive(PartialEq, Clone)]
pub struct WorldMapCanvasCache {
    terrain_canvas: HtmlCanvasElement,
    region_center_canvas: HtmlCanvasElement,
    super_region_grid_canvas: HtmlCanvasElement,
    region_grid_canvas: HtmlCanvasElement,
    map_id_indicator_canvas: HtmlCanvasElement,
}

impl WorldMapCanvasCache {
    pub fn new() -> Self {
        Self {
            terrain_canvas: create_canvas(),
            region_center_canvas: create_canvas(),
            super_region_grid_canvas: create_canvas(),
            region_grid_canvas: create_canvas(),
            map_id_indicator_canvas: create_canvas(),
        }
    }

    fn clear_helper(canvas: &HtmlCanvasElement, context: &web_sys::CanvasRenderingContext2d) {
        context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
    }

    fn clear_canvas(canvas: &HtmlCanvasElement) {
        let context = get_canvas_context(canvas);
        Self::clear_helper(canvas, &context);
    }

    pub fn resize_all(&self, view_context: &WorldMapContext, main_canvas: &HtmlCanvasElement) {
        let width = view_context.viewport_width;
        let height = view_context.viewport_height;

        let resize = |canvas: &HtmlCanvasElement| {
            canvas.set_width(width);
            canvas.set_height(height);
        };

        resize(main_canvas);
        resize(&self.terrain_canvas);
        resize(&self.region_center_canvas);
        resize(&self.region_grid_canvas);
        resize(&self.super_region_grid_canvas);
        resize(&self.map_id_indicator_canvas);
    }

    pub fn pre_render(&self, game: &Game, view_context: &WorldMapContext) {
        self.render_terrain_map(game, view_context, &self.terrain_canvas);
        self.render_region_centers(game, view_context, &self.region_center_canvas);
        self.render_region_grid(game, view_context, &self.region_grid_canvas);
        self.render_super_region_grid(game, view_context, &self.super_region_grid_canvas);
        self.render_map_id_indicator(game, view_context, &self.map_id_indicator_canvas);
    }

    pub fn clear_all(&self) {
        Self::clear_canvas(&self.terrain_canvas);
        Self::clear_canvas(&self.region_center_canvas);
        Self::clear_canvas(&self.region_grid_canvas);
        Self::clear_canvas(&self.super_region_grid_canvas);
        Self::clear_canvas(&self.map_id_indicator_canvas);
    }

    pub fn re_render_center_map(&self, game: &Game, view_context: &WorldMapContext) {
        let canvas = &self.region_center_canvas;
        let context = get_canvas_context(canvas);
        Self::clear_helper(canvas, &context);
        self.render_region_centers(game, view_context, canvas);
    }

    pub fn re_render_map_id_indicator(&self, game: &Game, view_context: &WorldMapContext) {
        let canvas = &self.map_id_indicator_canvas;
        let context = get_canvas_context(canvas);
        Self::clear_helper(canvas, &context);

        self.render_map_id_indicator(game, view_context, canvas);
    }

    pub fn render(&self, game: &Game, view_context: &WorldMapContext, canvas: &HtmlCanvasElement) {
        if view_context.viewport_height == 0 || view_context.viewport_width == 0 {
            // don't render anything
            return;
        }

        let context = get_canvas_context(canvas);

        let render = |canvas: &HtmlCanvasElement| {
            context
                .draw_image_with_html_canvas_element(canvas, 0., 0.)
                .unwrap();
        };

        context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);

        render(&self.terrain_canvas);
        self.draw_faction(game, view_context, canvas);
        render(&self.region_center_canvas);
        render(&self.region_grid_canvas);
        render(&self.super_region_grid_canvas);
        self.draw_active_top_left(game, view_context, canvas);
        render(&self.map_id_indicator_canvas);

        if let Some(world_map_hover_tile_id) = view_context.hover_tile_id {
            self.draw_hover(
                game,
                view_context,
                canvas,
                view_context.map_id_indicator_type,
                world_map_hover_tile_id,
            )
        }
    }
}
