mod command;
mod input;
mod logs;

use yew::prelude::*;

use crate::{
    context::{console::ConsoleAction, ControllerAction, ControllerType},
    view::console::{input::Input, logs::Logs},
};

#[function_component]
pub fn Console() -> Html {
    let view_context = use_context::<ControllerType>().expect("no context found");

    let onkeydown = {
        let dispatcher = view_context.dispatcher();

        move |e: KeyboardEvent| {
            let key_code = e.code();
            if key_code == "Enter" || key_code == "NumpadEnter" {
                dispatcher.dispatch(ControllerAction::Console(ConsoleAction::FlushBuffer));
            }
        }
    };

    html! {
        <label class="console" {onkeydown} for="console-input">
            <Logs />
            <div class="input-container buffer">
                {">"} <Input />
            </div>
        </label>
    }
}
