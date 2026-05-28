use crate::model::game::Game;

pub fn handle_auto_run_start(game: &mut Game) {
    game.auto_run_mut().start_input();
}

pub fn handle_auto_run_digit(game: &mut Game, d: char) {
    game.auto_run_mut().push_digit(d);
}

pub fn handle_auto_run_backspace(game: &mut Game) {
    game.auto_run_mut().pop_digit();
}

pub fn handle_auto_run_confirm(game: &mut Game) {
    if !game.auto_run_mut().confirm() {
        game.auto_run_mut().cancel();
    }
}

pub fn handle_auto_run_cancel(game: &mut Game) {
    game.auto_run_mut().cancel();
}
