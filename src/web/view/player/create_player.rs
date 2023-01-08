use yew::prelude::*;

use crate::{
    component::text_input::TextInput,
    context::{ControllerAction, ControllerType, GameAction},
};

#[derive(PartialEq, Properties)]
pub struct CreatePlayerProps {}

#[function_component]
pub fn CreatePlayer(CreatePlayerProps { .. }: &CreatePlayerProps) -> Html {
    let controller_handle = use_context::<ControllerType>().expect("no context found");
    let first_name_state = use_state(|| "".to_string());
    let family_name_state = use_state(|| "".to_string());

    let onsubmit = {
        let first_name_state = first_name_state.clone();
        let family_name_state = family_name_state.clone();

        move |e: SubmitEvent| {
            e.prevent_default();
            let first_name = first_name_state.to_string();
            let family_name = family_name_state.to_string();
            controller_handle.dispatch(ControllerAction::Game(GameAction::SpawnPlayer {
                first_name,
                family_name,
            }));
        }
    };

    html! {
        <form onsubmit={onsubmit} class="create-player-form">
            <div class="question-container">
                <label>
                    {"First Name"}
                    <TextInput name="first-name" handle={first_name_state} required={true}/>
                </label>

                <label>
                    {"Family Name"}
                    <TextInput name="family-name" handle={family_name_state} required={true}/>
                </label>
            </div>

            <div class="command-group">
                <button type="submit">{"Create"}</button>
                <button type="reset">{"Reset"}</button>
            </div>
        </form>
    }
}
