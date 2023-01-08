use yew::prelude::*;

use crate::{context::ControllerType, view::console::command::get_character::GetCharacterView};

#[function_component]
pub fn GetPlayerView() -> Html {
    let controller_handle = use_context::<ControllerType>().expect("no context found");
    let controller = controller_handle.borrow();
    let game = &controller.game;

    if let Some(id) = game.player {
        html! {
            <div>
                <div>{"Found player"}</div>
                <GetCharacterView {id}></GetCharacterView>
            </div>
        }
    } else {
        html! {
            <div>{"No Player"}</div>
        }
    }
}
