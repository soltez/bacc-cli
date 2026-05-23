use crate::model::game::Game;

pub fn toggle_show_hands(game: &mut Game) {
    game.display_mut().toggle_show_hands();
}

pub fn toggle_peel_enabled(game: &mut Game) {
    game.toggle_peel_enabled();
}
