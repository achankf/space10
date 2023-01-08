use yew::prelude::*;

use crate::context::{world_map::WorldMapAction, ControllerAction, ControllerType};

#[function_component]
pub fn ShowIdCheckbox() -> Html {
    let controller_handle = use_context::<ControllerType>().expect("no context found");
    let controller = controller_handle.borrow();

    let checked = (controller.world_map).is_show_map_id;

    let onclick = {
        let dispatcher = controller_handle.dispatcher();
        move |_| {
            dispatcher.dispatch(ControllerAction::WorldMap(WorldMapAction::SetIsShowMapId(
                !checked,
            )))
        }
    };

    html! {
        <label for="is-show-map-id">
            <input type="checkbox" id="is-show-map-id" {checked} {onclick} />
            {"Show id?"}
        </label>
    }
}
