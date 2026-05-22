use crate::model::game::Game;

pub fn deal_hand(game: &mut Game) {
    let Some(round) = game.next_shoe_round() else {
        return;
    };
    game.round_mut().start(round);
}

pub fn advance_deal(game: &mut Game) {
    let peel_enabled = game.display().peel_enabled();
    game.round_mut().advance_phase(peel_enabled);
}
