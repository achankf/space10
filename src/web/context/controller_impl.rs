use std::rc::Rc;

use yew::prelude::*;

use crate::utils::assign_if_ne::assign_if_ne;

use super::{
    console::ConsoleContext, game, world_map::WorldMapContext, Controller, ControllerAction,
    SubView, ViewKind, WrappedSubContext,
};

impl Reducible for WrappedSubContext<Controller> {
    /// Reducer Action Type
    type Action = ControllerAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let is_dirty = {
            let mut next_state = self.borrow_mut();

            match action {
                Self::Action::Switch(view) => assign_if_ne(&mut (*next_state).current, view),
                Self::Action::WorldMap(action) => {
                    WorldMapContext::inplace_update(&mut next_state, action)
                }
                Self::Action::Keydown(key) => {
                    if next_state.current != ViewKind::Console && key.as_str() == "Backquote" {
                        next_state.current = ViewKind::Console;
                        true
                    } else {
                        /*
                        match next_state.current {
                            ViewKind::WorldMap => next_state.world_map_context.reduce_with_key(key),
                            ViewKind::Map | ViewKind::Player | ViewKind::Console => {}
                        };
                        */
                        false
                    }
                }
                Self::Action::Console(action) => {
                    ConsoleContext::inplace_update(&mut next_state, action)
                }
                ControllerAction::Game(action) => {
                    game::game_inplace_update(&mut next_state.game, action);
                    true // updating the Game model is always going to be dirty
                }
            }
        };

        let generation = if is_dirty {
            self.generation.checked_add(1).unwrap_or_default()
        } else {
            self.generation
        };

        Self {
            generation,
            src: self.as_ref().src.clone(),
        }
        .into()
    }
}
