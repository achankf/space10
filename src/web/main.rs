mod component;
mod context;
mod enum_types;
mod parser;
mod utils;
mod view;

#[macro_use]
extern crate pest_derive;

use crate::context::world_map::WorldMapContext;
use crate::context::Controller;
use crate::context::ControllerAction;
use crate::context::ControllerType;
use crate::context::WrappedSubContext;
use crate::utils::html::window;
use crate::utils::panic_hook::set_panic_hook;
use crate::view::game_layout::GameLayout;

use space10::Game;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let controller_handle = use_reducer(|| {
        let mut game = Game::allocate(5, 5);
        game.setup_world();
        let world_map = WorldMapContext::new(&game);
        let controller = Controller {
            game,
            console: Default::default(),
            world_map,
            current: Default::default(),
        };

        WrappedSubContext::new(controller)
    });

    use_effect({
        let dispatcher = controller_handle.dispatcher();
        move || {
            let f = {
                Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
                    let key_code = e.code();
                    dispatcher.dispatch(ControllerAction::Keydown(key_code));
                })
            };
            window()
                .add_event_listener_with_callback("keydown", f.as_ref().unchecked_ref())
                .unwrap();

            move || {
                window()
                    .remove_event_listener_with_callback("keydown", f.as_ref().unchecked_ref())
                    .unwrap();
            }
        }
    });

    type ControllerProvider = ContextProvider<ControllerType>;

    html! {
        <ControllerProvider context={controller_handle}>
            <GameLayout></GameLayout>
        </ControllerProvider>
    }
}

fn main() {
    set_panic_hook();
    yew::Renderer::<App>::new().render();
}
