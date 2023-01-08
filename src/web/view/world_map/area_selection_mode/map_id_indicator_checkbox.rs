use yew::prelude::*;

use crate::{
    context::{world_map::WorldMapAction, ControllerAction, ControllerType},
    enum_types::map_id_indicator_kind::MapIdIndicatorKind,
};

#[derive(PartialEq, Properties)]
pub struct CheckboxProps {
    pub r#type: MapIdIndicatorKind,
}

#[function_component]
pub fn MapIdIndicatorCheckbox(CheckboxProps { r#type }: &CheckboxProps) -> Html {
    let controller_handle = use_context::<ControllerType>().expect("no context found");
    let controller = controller_handle.borrow();

    let cur_indicator_type = (controller.world_map).map_id_indicator_type;

    let onclick = {
        let dispatcher = controller_handle.dispatcher();
        let r#type = *r#type;
        move |_| {
            dispatcher.dispatch(ControllerAction::WorldMap(
                WorldMapAction::SetMapIdIndicatorType(r#type),
            ))
        }
    };

    let checked = cur_indicator_type == *r#type;
    let name = r#type.id_name();

    html! {
        <label for={name}>
            <input type="radio" name="map-id-indicator" id={name}
                {checked} {onclick}
            />
            {r#type}
        </label>
    }
}
