use std::{cell::RefCell, rc::Rc};

use space10::Game;
use yew::UseReducerHandle;

use self::{
    console::{ConsoleAction, ConsoleContext},
    world_map::{WorldMapAction, WorldMapContext},
};

pub(crate) mod console;
pub(crate) mod controller_impl;
pub(crate) mod game;
pub(crate) mod world_map;
pub(crate) mod wrapped_sub_context;

pub struct WrappedSubContext<T> {
    src: Rc<RefCell<T>>,
    generation: u8,
}

pub enum GameAction {
    SpawnPlayer {
        first_name: String,
        family_name: String,
    },
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ViewKind {
    #[default]
    WorldMap,
    Map,
    Player,
    Console,
}

pub enum ControllerAction {
    Game(GameAction), // model

    // global-views
    Switch(ViewKind),
    Keydown(String),

    // sub-views
    Console(ConsoleAction),
    WorldMap(WorldMapAction),
}

pub struct Controller {
    pub game: Game,
    pub console: ConsoleContext,
    pub world_map: WorldMapContext,
    pub current: ViewKind,
}

pub type ControllerType = UseReducerHandle<WrappedSubContext<Controller>>;

pub trait SubView {
    type Action;

    /// perform an in-place update and returns a "dirty" flag which indiciates whether the action really changed the object.
    fn inplace_update(controller: &mut Controller, action: Self::Action) -> bool;
}
