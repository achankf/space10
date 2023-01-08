use space10::{Game, TileId};
use web_sys::{HtmlCanvasElement, MouseEvent};
use yew::prelude::*;

use crate::{
    enum_types::map_id_indicator_kind::MapIdIndicatorKind, utils::assign_if_ne::assign_if_ne,
};

use super::{Controller, SubView};

#[derive(PartialEq, Clone)]
pub struct WorldMapContext {
    /** The tile id that is mapped to the origin (0,0) of the camera */
    pub camera_origin: TileId,
    pub selected_tile_id: TileId,

    /** need to be an odd value for a middle y */
    pub map_view_columns: u32,
    /** need to be an odd value for a middle x */
    pub map_view_rows: u32,

    /** value in pixel */
    pub tile_scale_factor: u32,

    pub hover_tile_id: Option<TileId>,

    pub is_show_region_center: bool,

    pub is_show_map_id: bool,

    pub has_prerendered: bool,

    pub map_id_indicator_type: MapIdIndicatorKind,

    pub viewport_center: TileId,
    pub viewport_width: u32,
    pub viewport_height: u32,
}

#[derive(Clone, Debug)]
pub enum WorldMapAction {
    SetShowRegionCenter(bool),
    SetMapIdIndicatorType(MapIdIndicatorKind),
    SetIsShowMapId(bool),
    SetWorldMapHoverTileId(Option<TileId>),
    TryZoomIn { canvas_ref: NodeRef, e: WheelEvent },
    TryZoomOut { canvas_ref: NodeRef, e: WheelEvent },
    Resize { width: u32, height: u32 },
}

impl WorldMapContext {
    pub fn get_viewport_coor(canvas: &HtmlCanvasElement, e: &MouseEvent) -> (f64, f64) {
        let rect = canvas.get_bounding_client_rect();
        let x = e.client_x() as f64 - rect.left();
        let y = e.client_y() as f64 - rect.top();

        (x, y)
    }

    pub fn viewport_to_game_coor(&self, game: &Game, (x, y): (f64, f64)) -> Option<(usize, usize)> {
        let tile_scale_factor = self.tile_scale_factor;
        let max_width = game.get_num_columns() as u32 * tile_scale_factor;
        let max_height = game.get_num_rows() as u32 * tile_scale_factor;

        if x > max_width as f64 || y > max_height as f64 {
            None
        } else {
            let tx = (x / tile_scale_factor as f64).floor();
            let ty = (y / tile_scale_factor as f64).floor();

            Some((tx as usize, ty as usize))
        }
    }

    pub fn viewport_to_tile_coor(
        &self,
        game: &Game,
        canvas: &HtmlCanvasElement,
        e: &MouseEvent,
    ) -> Option<(usize, usize)> {
        let vp_coor = Self::get_viewport_coor(&canvas, &e);
        self.viewport_to_game_coor(&game, vp_coor)
    }

    pub fn new(game: &Game) -> Self {
        let map_view_columns: u32 = 17;
        let map_view_rows: u32 = 9;

        let camera_origin_tile_id = {
            let zone_id = game.get_random_faction().get_capital_zone();
            let tile_id = game.to_zone_center_id(zone_id);

            game.project_center_to_camera_origin(
                tile_id,
                map_view_rows as usize,
                map_view_columns as usize,
            )
        };

        Self {
            map_view_columns,
            map_view_rows,
            tile_scale_factor: 1,
            camera_origin: camera_origin_tile_id,
            selected_tile_id: Default::default(),
            hover_tile_id: Default::default(),
            is_show_region_center: Default::default(),
            is_show_map_id: Default::default(),
            has_prerendered: Default::default(),
            map_id_indicator_type: Default::default(),
            viewport_center: Default::default(),
            viewport_width: Default::default(),
            viewport_height: Default::default(),
        }
    }

    pub fn get_camera_origin(&self) -> TileId {
        self.camera_origin
    }
}

impl SubView for WorldMapContext {
    type Action = WorldMapAction;

    fn inplace_update(controller: &mut Controller, action: Self::Action) -> bool {
        let world_map = &mut controller.world_map;
        let game = &mut controller.game;

        match action {
            WorldMapAction::SetShowRegionCenter(value) => {
                assign_if_ne(&mut world_map.is_show_region_center, value)
            }
            WorldMapAction::SetMapIdIndicatorType(value) => {
                assign_if_ne(&mut world_map.map_id_indicator_type, value)
            }
            WorldMapAction::SetIsShowMapId(value) => {
                assign_if_ne(&mut world_map.is_show_map_id, value)
            }
            WorldMapAction::SetWorldMapHoverTileId(tile_id) => {
                assign_if_ne(&mut world_map.hover_tile_id, tile_id)
            }
            WorldMapAction::TryZoomOut { canvas_ref, e } => {
                let tile_id = {
                    let canvas = canvas_ref.cast().unwrap();
                    let tile_coor = world_map.viewport_to_tile_coor(game, &canvas, &e);
                    tile_coor.map(|coor| game.to_tile_id(coor))
                };

                if let Some(tile_id) = tile_id {
                    let tile_scale_factor = (world_map.tile_scale_factor - 1).max(1);

                    let a1 = assign_if_ne(&mut world_map.tile_scale_factor, tile_scale_factor);
                    let a2 = assign_if_ne(&mut world_map.camera_origin, tile_id);

                    a1 && a2
                } else {
                    false
                }
            }
            WorldMapAction::TryZoomIn { canvas_ref, e } => {
                let tile_id = {
                    let canvas = canvas_ref.cast().unwrap();
                    let tile_coor = world_map.viewport_to_tile_coor(game, &canvas, &e);
                    tile_coor.map(|coor| game.to_tile_id(coor))
                };

                if let Some(tile_id) = tile_id {
                    let tile_scale_factor = world_map.tile_scale_factor + 1;

                    let a1 = assign_if_ne(&mut world_map.tile_scale_factor, tile_scale_factor);
                    let a2 = assign_if_ne(&mut world_map.camera_origin, tile_id);

                    a1 && a2
                } else {
                    false
                }
            }
            WorldMapAction::Resize { width, height } => {
                let a1 = assign_if_ne(&mut world_map.viewport_width, width);
                let a2 = assign_if_ne(&mut world_map.viewport_height, height);

                a1 & a2
            }
        }
    }
}
