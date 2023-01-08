mod get_character;
mod get_player;

use yew::prelude::*;

use crate::{
    parser::command::Command,
    view::console::command::{get_character::GetCharacterView, get_player::GetPlayerView},
};

#[derive(Clone, PartialEq, Properties)]
pub struct CommandProps {
    pub command: Command,
}

#[function_component]
pub fn CommandView(props: &CommandProps) -> Html {
    match &props.command {
        &Command::GetCharacter(id) => {
            html! { <GetCharacterView id={id}></GetCharacterView> }
        }
        Command::GetPlayer => {
            html! { <GetPlayerView></GetPlayerView> }
        }
        command @ _ => {
            html! {
                <div>{format!("{command:?}")}</div>
            }
        }
    }
}
