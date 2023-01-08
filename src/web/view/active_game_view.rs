use yew::prelude::*;

use crate::{
    context::{ControllerType, ViewKind},
    view::{console::Console, map::Map, player::show_player::ShowPlayer, world_map::WorldMap},
};

#[derive(PartialEq, Properties)]
pub struct ActiveGameViewProps {}

#[function_component]
pub fn ActiveGameView(ActiveGameViewProps { .. }: &ActiveGameViewProps) -> Html {
    let controller_handle = use_context::<ControllerType>().expect("no context found");
    let controller = controller_handle.borrow();

    match controller.current {
        ViewKind::WorldMap => html! { <WorldMap /> },
        ViewKind::Map => html! { <Map /> },
        ViewKind::Player => html! {<ShowPlayer/>},
        ViewKind::Console => html! {<Console/>},
    }
}
