use js_sys::JSON;
use web_sys::console;
use yew::prelude::*;

use crate::{context::ControllerType, view::player::create_player::CreatePlayer};

#[derive(PartialEq, Properties)]
pub struct CreatePlayerProps {}

#[function_component]
pub fn ShowPlayer(CreatePlayerProps { .. }: &CreatePlayerProps) -> Html {
    let controller_handle = use_context::<ControllerType>().expect("no context found");
    let controller = controller_handle.borrow();

    if let Some(player) = controller.game.get_player() {
        let player_str = serde_json::to_string_pretty(&player).unwrap();

        let player_json = JSON::parse(&player_str).unwrap();
        console::log_1(&player_json);

        html! {
            <div>{player_str}</div>
        }
    } else {
        html! {
            <CreatePlayer></CreatePlayer>
        }
    }
}
