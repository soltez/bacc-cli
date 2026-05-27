use crate::model::bet::BetPick;
use crate::model::game::Game;

pub fn handle_bet_key(game: &mut Game, ch: char) {
    match ch {
        'p' | 'P' => game.bet_mut().start_input(BetPick::Player),
        't' | 'T' => game.bet_mut().start_input(BetPick::Tie),
        'b' | 'B' => game.bet_mut().start_input(BetPick::Banker),
        'c' | 'C' => game.bet_mut().clear(),
        '0'..='9' => game.bet_mut().push_digit(ch),
        _ => {}
    }
}

pub fn handle_bet_backspace(game: &mut Game) {
    game.bet_mut().pop_digit();
}

pub fn handle_bet_enter(game: &mut Game) {
    game.bet_mut().confirm();
}

pub fn handle_bet_escape(game: &mut Game) {
    game.bet_mut().cancel_input();
}
