use yew::prelude::*;

use crate::context::{ControllerAction, ControllerType, ViewKind};

#[derive(PartialEq, Properties)]
pub struct NavProps {
    pub symbol: char,
    pub description: String,
    pub view: ViewKind,
}

#[function_component]
pub fn NavItem(
    NavProps {
        symbol,
        description,
        view,
    }: &NavProps,
) -> Html {
    let controller_handle = use_context::<ControllerType>().expect("no context found");
    let controller = controller_handle.borrow();

    let onclick = {
        let dispatcher = controller_handle.dispatcher();
        let view = view.clone();
        move |_: MouseEvent| dispatcher.dispatch(ControllerAction::Switch(view.clone()))
    };

    let selected = if controller.current == *view {
        "selected"
    } else {
        ""
    };

    html! {
        <div class={classes!("nav-item", selected)}>
          <button type="button" class={"item-link"} {onclick}>
            <div class="symbol">{symbol}</div>
            <div class="description">{description}</div>
          </button>
        </div>
    }
}
