use bacc::{BaccaratRound, BaccaratScoreboard};

use super::display_options::DisplayOptions;
use super::round_state::RoundState;
use super::scoreboard::ScoreboardCache;
use super::shoe::Shoe;

pub struct Game {
    shoe: Shoe,
    display: DisplayOptions,
    round: RoundState,
    scoreboard: BaccaratScoreboard,
    cache: ScoreboardCache,
    pending_round: Option<BaccaratRound>,
    shoe_number: u32,
    peel_enabled: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            shoe: Shoe::new(),
            display: DisplayOptions::new(),
            round: RoundState::new(),
            scoreboard: BaccaratScoreboard::new(),
            cache: ScoreboardCache::new(),
            pending_round: None,
            shoe_number: 1,
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

    pub fn scoreboard_mut(&mut self) -> &mut BaccaratScoreboard {
        &mut self.scoreboard
    }

    pub fn scoreboard_cache(&self) -> &ScoreboardCache {
        &self.cache
    }

    pub fn scoreboard_cache_mut(&mut self) -> &mut ScoreboardCache {
        &mut self.cache
    }

    pub fn update_scoreboard(&mut self, round: &BaccaratRound) {
        self.scoreboard.update(round);
        self.cache.update(&self.scoreboard);
    }

    pub fn store_pending_round(&mut self, round: BaccaratRound) {
        self.pending_round = Some(round);
    }

    pub fn take_pending_round(&mut self) -> Option<BaccaratRound> {
        self.pending_round.take()
    }

    pub fn shoe_number(&self) -> u32 {
        self.shoe_number
    }

    pub fn increment_shoe_number(&mut self) {
        self.shoe_number += 1;
    }

    pub fn next_shoe_round(&mut self) -> Option<BaccaratRound> {
        self.shoe.next_round()
    }
}
