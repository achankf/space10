mod render_map;

use web_sys::HtmlCanvasElement;
use yew::prelude::*;

use crate::context::ControllerType;

#[function_component]
pub fn Map() -> Html {
    let controller_handle = use_context::<ControllerType>().expect("no context found");
    let canvas_ref = use_node_ref();

    use_effect({
        let controller_handle = controller_handle.clone();
        let canvas_ref = canvas_ref.clone();

        move || {
            let canvas: HtmlCanvasElement = canvas_ref.cast().expect("canvas not attached");
            render_map::render_map(&controller_handle.borrow(), &canvas);
        }
    });

    html! {
        <div class="map-container">
            <canvas ref={canvas_ref}></canvas>
        </div>
    }
}
