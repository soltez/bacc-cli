use bacc::BaccaratRound;

use super::display_options::DisplayOptions;
use super::round_state::RoundState;
use super::shoe::Shoe;

pub struct Game {
    shoe: Shoe,
    display: DisplayOptions,
    round: RoundState,
}

impl Game {
    pub fn new() -> Self {
        Game {
            shoe: Shoe::new(),
            display: DisplayOptions::new(),
            round: RoundState::new(),
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

    pub fn next_shoe_round(&mut self) -> Option<BaccaratRound> {
        self.shoe.next_round()
    }
}
