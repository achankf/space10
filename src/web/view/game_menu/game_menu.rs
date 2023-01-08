use yew::prelude::*;

use crate::context::ViewKind;
use crate::view::game_menu::nav_item::NavItem;
use crate::view::game_menu::section::Section;

#[function_component]
pub fn GameMenu() -> Html {
    html! {
        <div class="game-menu">
            <Section>
                <div class="title">
                    <span class="emoji">{'🗺'}</span> {"Chessboard"}
                </div>
                <NavItem view={ViewKind::WorldMap} symbol='天' description="World" />
                <NavItem view={ViewKind::Map} symbol='地' description="Local" />
                <NavItem view={ViewKind::Player} symbol='人' description="Player" />
            </Section>
        </div>
    }
}
