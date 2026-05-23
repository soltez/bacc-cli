use crate::model::game::Game;

pub fn deal_hand(game: &mut Game) {
    let Some(round) = game.next_shoe_round() else {
        return;
    };
    game.round_mut().start(round);
}

pub fn advance_deal(game: &mut Game) {
    let peel_enabled = game.peel_enabled();
    game.round_mut().advance_phase(peel_enabled);
}

pub fn end_round(game: &mut Game) {
    game.round_mut().end_round();
}

pub fn should_auto_advance(game: &Game) -> bool {
    game.round().should_auto_advance()
}

pub fn auto_advance_delay_ms(game: &Game) -> u64 {
    game.display().deal_speed_ms()
}

pub fn handle_enter(game: &mut Game) {
    if game.round().phase() != 0 && game.round().complete() {
        end_round(game);
    } else if game.round().phase() == 0 {
        deal_hand(game);
    } else {
        advance_deal(game);
    }
}
