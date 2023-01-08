pub(crate) mod area_selection_mode;
pub(crate) mod canvas_cache;
pub(crate) mod map_canvas;
pub(crate) mod show_region_center_checkbox;

use yew::prelude::*;

use crate::view::world_map::{
    area_selection_mode::area_selection_mode::AreaSelectionMode, map_canvas::MapCanvas,
    show_region_center_checkbox::ShowRegionCenterCheckbox,
};

#[function_component]
pub fn WorldMap() -> Html {
    html! {
        <div class="world-map-container">
            <div class="options">
                <AreaSelectionMode />
                <div>
                    <ShowRegionCenterCheckbox />
                </div>
            </div>
            <MapCanvas />
        </div>
    }
}
