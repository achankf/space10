use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{
    context::{world_map::WorldMapAction, ControllerAction, ControllerType},
    utils::html::window,
    view::world_map::canvas_cache::WorldMapCanvasCache,
};

#[function_component]
pub fn MapCanvas() -> Html {
    let controller_handle = use_context::<ControllerType>().expect("no context found");
    let canvas_cache = {
        let _controller_handle = controller_handle.clone();

        use_memo(|_| WorldMapCanvasCache::new(), ())
    };
    let canvas_ref = use_node_ref();
    let container_ref = use_node_ref();

    use_effect_with_deps(
        {
            let view_dispatcher = controller_handle.dispatcher();
            let container_ref = container_ref.clone();

            move |_| {
                let f = {
                    let container = container_ref.cast::<HtmlElement>().unwrap();
                    let view_dispatcher = view_dispatcher.clone();
                    Closure::<dyn FnMut(JsValue)>::new(move |_| {
                        let rect = container.get_bounding_client_rect();
                        let (width, height) = (rect.width() as u32, rect.height() as u32);
                        view_dispatcher.dispatch(ControllerAction::WorldMap(
                            WorldMapAction::Resize { width, height },
                        ));
                    })
                };
                window()
                    .add_event_listener_with_callback("resize", f.as_ref().unchecked_ref())
                    .unwrap();

                let container = container_ref.cast::<HtmlElement>().unwrap();
                let rect = container.get_bounding_client_rect();
                let (width, height) = (rect.width() as u32, rect.height() as u32);
                view_dispatcher.dispatch(ControllerAction::WorldMap(WorldMapAction::Resize {
                    width,
                    height,
                }));

                move || {
                    window()
                        .remove_event_listener_with_callback("resize", f.as_ref().unchecked_ref())
                        .unwrap();
                }
            }
        },
        (),
    );

    use_effect_with_deps(
        {
            let canvas_cache = canvas_cache.clone();
            let canvas_ref = canvas_ref.clone();
            let view_context = controller_handle.clone();

            move |_| {
                let canvas = canvas_ref.cast().expect("canvas not attached");
                let controller = &view_context.borrow();
                let world_map = &controller.world_map;
                let game = &controller.game;

                canvas_cache.resize_all(world_map, &canvas);
                canvas_cache.pre_render(game, world_map);
            }
        },
        {
            let context = &controller_handle.borrow().world_map;
            (context.viewport_width, context.viewport_height)
        },
    );

    // zooming
    use_effect_with_deps(
        {
            let canvas_cache = canvas_cache.clone();
            let controller_handle = controller_handle.clone();

            move |_| {
                canvas_cache.clear_all();
                let controller = controller_handle.borrow();
                canvas_cache.pre_render(&controller.game, &controller.world_map);
            }
        },
        {
            let controller_handle = controller_handle.borrow();
            controller_handle.world_map.tile_scale_factor
        },
    );

    // show region centers
    use_effect_with_deps(
        {
            let canvas_cache = canvas_cache.clone();
            let controller_handle = controller_handle.clone();
            move |_| {
                let controller = controller_handle.borrow();
                canvas_cache.re_render_center_map(&controller.game, &controller.world_map)
            }
        },
        controller_handle.borrow().world_map.is_show_region_center,
    );

    // show zone/region/super region ids
    use_effect_with_deps(
        {
            let canvas_cache = canvas_cache.clone();
            let controller_handle = controller_handle.clone();
            move |_| {
                let controller = controller_handle.borrow();
                canvas_cache.re_render_map_id_indicator(&controller.game, &controller.world_map)
            }
        },
        {
            let controller_handle = controller_handle.borrow();
            let world_map = &controller_handle.world_map;
            (world_map.is_show_map_id, world_map.map_id_indicator_type)
        },
    );

    // render the world map -- should be placed as the last render function
    use_effect({
        let canvas_cache = canvas_cache.clone();
        let canvas_ref = canvas_ref.clone();
        let controller_handle = controller_handle.clone();

        move || {
            let controller = controller_handle.borrow();

            canvas_cache.render(
                &controller.game,
                &controller.world_map,
                &canvas_ref.cast().expect("canvas not attached"),
            )
        }
    });

    let onmousemove = {
        let controller_handle = controller_handle.clone();
        let canvas_ref = canvas_ref.clone();
        move |e| {
            // note to self: if you need to borrow the controller for calculations, make sure
            //    the borrow isn't in the same scope as the dispatch call.

            let tile_id = {
                let controller = controller_handle.borrow();
                let canvas = canvas_ref.cast().unwrap();
                let game = &controller.game;
                let tile_coor = controller
                    .world_map
                    .viewport_to_tile_coor(game, &canvas, &e);
                tile_coor.map(|coor| game.to_tile_id(coor))
            };

            controller_handle.dispatch(ControllerAction::WorldMap(
                WorldMapAction::SetWorldMapHoverTileId(tile_id),
            ));
        }
    };

    let onmouseout = {
        let dispatcher = controller_handle.dispatcher();
        move |_| {
            dispatcher.dispatch(ControllerAction::WorldMap(
                WorldMapAction::SetWorldMapHoverTileId(None),
            ))
        }
    };

    let onwheel = {
        let canvas_ref = canvas_ref.clone();
        let dispatcher = controller_handle.dispatcher();

        move |e: WheelEvent| {
            let canvas_ref = canvas_ref.clone();
            if e.delta_y().is_sign_positive() {
                dispatcher.dispatch(ControllerAction::WorldMap(WorldMapAction::TryZoomOut {
                    canvas_ref,
                    e,
                }));
            } else {
                dispatcher.dispatch(ControllerAction::WorldMap(WorldMapAction::TryZoomIn {
                    canvas_ref,
                    e,
                }));
            }
        }
    };

    html! {
        <div class="canvas-container" ref={container_ref}>
            <canvas class="world-map" ref={canvas_ref} {onmousemove} {onmouseout} {onwheel}></canvas>
        </div>
    }
}
