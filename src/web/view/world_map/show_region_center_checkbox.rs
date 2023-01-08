use yew::prelude::*;

use crate::context::{world_map::WorldMapAction, ControllerAction, ControllerType};

#[function_component]
pub fn ShowRegionCenterCheckbox() -> Html {
    let controller_handle = use_context::<ControllerType>().expect("no context found");
    let controller = controller_handle.borrow();

    let checked = controller.world_map.is_show_region_center;

    let onclick = {
        let dispatcher = controller_handle.dispatcher();
        move |_| {
            dispatcher.dispatch(ControllerAction::WorldMap(
                WorldMapAction::SetShowRegionCenter(!checked),
            ))
        }
    };

    html! {
        <label htmlFor="is-show-region-center">
            <input type="checkbox" id="is-show-region-center" {checked} onclick={onclick } />
            {"Show region center?"}
        </label>
    }
}
