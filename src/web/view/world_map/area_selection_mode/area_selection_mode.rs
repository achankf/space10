use yew::prelude::*;

use crate::{
    enum_types::map_id_indicator_kind::MapIdIndicatorKind,
    view::world_map::area_selection_mode::{
        map_id_indicator_checkbox::MapIdIndicatorCheckbox, show_id_checkbox::ShowIdCheckbox,
    },
};

#[function_component]
pub fn AreaSelectionMode() -> Html {
    html! {
        <fieldset>
            <legend>{"Area Selection Mode"}</legend>
            <div class="area-selection-mode-container">
                <MapIdIndicatorCheckbox r#type={MapIdIndicatorKind::Zone} />
                <MapIdIndicatorCheckbox r#type={MapIdIndicatorKind::Region} />
                <MapIdIndicatorCheckbox r#type={MapIdIndicatorKind::SuperRegion} />
                <ShowIdCheckbox />
            </div>
        </fieldset>
    }
}
