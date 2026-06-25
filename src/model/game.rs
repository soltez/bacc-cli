use bacc_core::{BaccRound, BaccScoreboard};

use super::auto_run::AutoRun;
use super::bet::Bet;
use super::display_options::DisplayOptions;
use super::round_state::RoundState;
use super::shoe::Shoe;
use super::stats::Stats;

pub struct Game {
    shoe: Shoe,
    display: DisplayOptions,
    round: RoundState,
    scoreboard: BaccScoreboard,
    stats: Stats,
    bet: Bet,
    auto_run: AutoRun,
    pending_round: Option<BaccRound>,
    shoe_number: u32,
    peel_enabled: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            shoe: Shoe::new(),
            display: DisplayOptions::new(),
            round: RoundState::new(),
            scoreboard: BaccScoreboard::new(),
            stats: Stats::new(),
            bet: Bet::new(),
            auto_run: AutoRun::new(),
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

    pub fn scoreboard(&self) -> &BaccScoreboard {
        &self.scoreboard
    }

    pub fn scoreboard_mut(&mut self) -> &mut BaccScoreboard {
        &mut self.scoreboard
    }

    pub fn update_models(&mut self, round: &BaccRound) {
        self.scoreboard.update(round);
        self.stats.update(round, &self.scoreboard);
        self.bet.settle(round.outcome().marker());
    }

    pub fn bet(&self) -> &Bet {
        &self.bet
    }

    pub fn bet_mut(&mut self) -> &mut Bet {
        &mut self.bet
    }

    pub fn auto_run(&self) -> &AutoRun {
        &self.auto_run
    }

    pub fn auto_run_mut(&mut self) -> &mut AutoRun {
        &mut self.auto_run
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    pub fn store_pending_round(&mut self, round: BaccRound) {
        self.pending_round = Some(round);
    }

    pub fn take_pending_round(&mut self) -> Option<BaccRound> {
        self.pending_round.take()
    }

    pub fn shoe_number(&self) -> u32 {
        self.shoe_number
    }

    pub fn increment_shoe_number(&mut self) {
        self.shoe_number += 1;
    }

    pub fn next_shoe_round(&mut self) -> Option<BaccRound> {
        self.shoe.next_round()
    }

    pub fn reset_shoe(&mut self) {
        self.shoe.reset();
    }
}
