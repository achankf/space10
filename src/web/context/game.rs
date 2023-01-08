use space10::Game;

use super::GameAction;

pub fn game_inplace_update(game: &mut Game, action: GameAction) {
    match action {
        GameAction::SpawnPlayer {
            first_name,
            family_name,
        } => {
            game.spawn_player(first_name, family_name);
        }
    }
}
