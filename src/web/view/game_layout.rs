use stylist::css;
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::view::active_game_view::ActiveGameView;
use crate::view::game_menu::game_menu::GameMenu;

#[styled_component]
pub fn GameLayout() -> Html {
    let container = css!(
        r#"
  display: grid;
  gap: 1rem;
  grid-template-columns: 11rem 1fr;
  grid-template-rows: 100vh;

  & > * {
    overflow: auto;
  }
"#
    );

    html! {
        <div class={container}>
            <GameMenu />
            <ActiveGameView />
        </div>
    }
}
