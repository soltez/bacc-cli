use bacc::BaccaratRound;

use super::display_options::DisplayOptions;
use super::round_state::RoundState;
use super::shoe::Shoe;

pub struct Game {
    shoe: Shoe,
    display: DisplayOptions,
    round: RoundState,
    peel_enabled: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            shoe: Shoe::new(),
            display: DisplayOptions::new(),
            round: RoundState::new(),
            peel_enabled: false,
        }
    }

    pub fn round(&self) -> &RoundState {
        &self.round
    }

    pub fn round_mut(&mut self) -> &mut RoundState {
        &mut self.round
    }

    pub fn display(&self) -> &DisplayOptions {
        &self.display
    }

    pub fn display_mut(&mut self) -> &mut DisplayOptions {
        &mut self.display
    }

    pub fn peel_enabled(&self) -> bool {
        self.peel_enabled
    }

    pub fn toggle_peel_enabled(&mut self) {
        self.peel_enabled = !self.peel_enabled;
    }

    pub fn next_shoe_round(&mut self) -> Option<BaccaratRound> {
        self.shoe.next_round()
    }
}
