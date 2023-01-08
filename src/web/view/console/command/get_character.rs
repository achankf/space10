use space10::CharacterId;
use yew::prelude::*;

use crate::context::ControllerType;

#[derive(Clone, PartialEq, Properties)]
pub struct GetCharacterViewProps {
    pub id: CharacterId,
}

#[function_component]
pub fn GetCharacterView(props: &GetCharacterViewProps) -> Html {
    let controller_handle = use_context::<ControllerType>().expect("no context found");
    let controller = controller_handle.borrow();
    let game = &controller.game;
    let id = props.id;
    let character = &game[id];

    let character_str = serde_json::to_string_pretty(character).unwrap();
    html! {
        <div>
            <div>{format!("Get Character -> {id:?}")}</div>
            <pre>{character_str}</pre>
        </div>
    }
}
