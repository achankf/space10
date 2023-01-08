use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::context::{console::ConsoleAction, ControllerAction, ControllerType};

#[function_component]
pub fn Input() -> Html {
    let view_context = use_context::<ControllerType>().expect("no context found");

    let buffer = (*view_context).borrow().console.buffer.clone();

    let oninput = {
        move |e: InputEvent| {
            let input = e
                .target_dyn_into::<HtmlInputElement>()
                .expect("casting an html input element");

            view_context.dispatch(ControllerAction::Console(ConsoleAction::UpdateBuffer(
                input.value(),
            )));
        }
    };

    html! {
        <input id="console-input" type="text" class="input" value={buffer} {oninput}/>
    }
}
