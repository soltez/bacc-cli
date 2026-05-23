use bacc::BaccaratRound;
use kev::{CardInt, Rank};

use super::card_window::CardWindow;

fn is_paint(card: CardInt) -> bool {
    matches!(card.rank(), Rank::Jack | Rank::Queen | Rank::King)
}

pub struct RoundState {
    banker_cards: Vec<CardInt>,
    player_cards: Vec<CardInt>,
    banker_score: u8,
    player_score: u8,
    phase: u8,
    player_third: bool,
    banker_third: bool,
    forced_third: bool,
    windows: [Option<CardWindow>; 6],
}

impl RoundState {
    pub fn new() -> Self {
        RoundState {
            banker_cards: Vec::new(),
            player_cards: Vec::new(),
            banker_score: 0,
            player_score: 0,
            phase: 0,
            player_third: false,
            banker_third: false,
            forced_third: false,
            windows: [None; 6],
        }
    }

    pub fn start(&mut self, round: BaccaratRound) {
        let bits = round.encode();
        self.player_cards = round.player_cards().to_vec();
        self.banker_cards = round.banker_cards().to_vec();
        self.player_score = ((bits >> 8) & 0xF) as u8;
        self.banker_score = ((bits >> 12) & 0xF) as u8;
        self.player_third = (bits >> 4) & 1 == 1;
        self.banker_third = (bits >> 5) & 1 == 1;
        self.forced_third = round.is_forced_third();
        self.phase = 10;
        self.windows = [None; 6];
        self.windows[0] = Some(CardWindow::FaceDown);
    }

    pub fn end_round(&mut self) {
        self.phase = 0;
    }

    pub fn phase(&self) -> u8 {
        self.phase
    }

    pub fn windows(&self) -> &[Option<CardWindow>; 6] {
        &self.windows
    }

    pub fn player_cards(&self) -> &[CardInt] {
        &self.player_cards
    }

    pub fn banker_cards(&self) -> &[CardInt] {
        &self.banker_cards
    }

    pub fn player_score(&self) -> u8 {
        self.player_score
    }

    pub fn banker_score(&self) -> u8 {
        self.banker_score
    }

    pub fn has_player_result(&self) -> bool {
        [0, 2, 4]
            .iter()
            .take(self.player_cards.len())
            .all(|&i| self.windows[i] == Some(CardWindow::FaceUp))
    }

    pub fn has_banker_result(&self) -> bool {
        let banker_cards_up = [1, 3, 5]
            .iter()
            .take(self.banker_cards.len())
            .all(|&i| self.windows[i] == Some(CardWindow::FaceUp));

        (self.banker_score >= 7 || self.has_player_result()) && banker_cards_up
    }

    pub fn complete(&self) -> bool {
        self.has_player_result() && self.has_banker_result()
    }

    pub fn should_auto_advance(&self) -> bool {
        let auto_four = self.phase >= 10 && self.phase <= 12;
        let p0_paint = self.player_cards.first().is_some_and(|&c| is_paint(c));
        let b0_paint = self.banker_cards.first().is_some_and(|&c| is_paint(c));
        let p2_paint = self.player_cards.get(2).is_some_and(|&c| is_paint(c));
        let b2_paint = self.banker_cards.get(2).is_some_and(|&c| is_paint(c));
        let auto_peel_reveal = self.phase == 42
            || self.phase == 43
            || (self.phase == 40 && p0_paint)
            || (self.phase == 41 && b0_paint);
        let auto_any_third = self.phase == 48 && (self.player_third || self.banker_third);
        let auto_forced_third = self.phase == 114 && self.forced_third;
        let auto_banker_third = self.phase == 144 && self.banker_third && !self.forced_third;
        let auto_p2_reveal = self.phase == 124 && p2_paint;
        let auto_b2_reveal = self.phase == 125 && b2_paint;
        auto_four
            || auto_peel_reveal
            || auto_any_third
            || auto_forced_third
            || auto_banker_third
            || auto_p2_reveal
            || auto_b2_reveal
    }

    pub fn advance_phase(&mut self, peel_enabled: bool) {
        let p0_paint = self.player_cards.first().is_some_and(|&c| is_paint(c));
        let p1_paint = self.player_cards.get(1).is_some_and(|&c| is_paint(c));
        let p2_paint = self.player_cards.get(2).is_some_and(|&c| is_paint(c));
        let b0_paint = self.banker_cards.first().is_some_and(|&c| is_paint(c));
        let b1_paint = self.banker_cards.get(1).is_some_and(|&c| is_paint(c));
        let b2_paint = self.banker_cards.get(2).is_some_and(|&c| is_paint(c));

        match self.phase {
            10 => {
                self.windows[1] = Some(CardWindow::FaceDown);
                self.phase = 11;
            }
            11 => {
                self.windows[2] = Some(CardWindow::FaceDown);
                self.phase = 12;
            }
            12 => {
                self.windows[3] = Some(CardWindow::FaceDown);
                self.phase = 13;
            }

            // Escape: any peel phase when peel disabled
            13..=46 if !peel_enabled => {
                for i in 0..4 {
                    self.windows[i] = Some(CardWindow::FaceUp);
                }
                self.phase = 48;
            }

            // Player initial card peel sequence
            13 => {
                self.windows[0] = Some(CardWindow::CornerPeel);
                self.windows[2] = Some(CardWindow::CornerPeel);
                self.phase = 26;
            }
            26 if !p0_paint && !p1_paint => {
                self.windows[0] = Some(CardWindow::SidePeel);
                self.windows[2] = Some(CardWindow::SidePeel);
                self.phase = 36;
            }
            26 if p1_paint && !p0_paint => {
                self.windows[2] = Some(CardWindow::FaceUp);
                self.phase = 42;
            }
            26 if p0_paint && !p1_paint => {
                self.windows[0] = Some(CardWindow::FaceUp);
                self.phase = 40;
            }
            26 => {
                self.windows[0] = Some(CardWindow::FaceUp);
                self.windows[2] = Some(CardWindow::FaceUp);
                self.phase = 46;
            }
            42 => {
                self.windows[0] = Some(CardWindow::SidePeel);
                self.phase = 30;
            }
            36 => {
                self.windows[0] = Some(CardWindow::FaceUp);
                self.phase = 40;
            }
            40 if p0_paint => {
                self.windows[2] = Some(CardWindow::SidePeel);
                self.phase = 32;
            }
            30 | 32 => {
                self.windows[0] = Some(CardWindow::FaceUp);
                self.windows[2] = Some(CardWindow::FaceUp);
                self.phase = 46;
            }
            40 => {
                self.windows[0] = Some(CardWindow::FaceUp);
                self.windows[2] = Some(CardWindow::FaceUp);
                self.phase = 46;
            }

            // Banker initial card peel sequence
            46 => {
                self.windows[1] = Some(CardWindow::CornerPeel);
                self.windows[3] = Some(CardWindow::CornerPeel);
                self.phase = 27;
            }
            27 if !b0_paint && !b1_paint => {
                self.windows[1] = Some(CardWindow::SidePeel);
                self.windows[3] = Some(CardWindow::SidePeel);
                self.phase = 37;
            }
            27 if b1_paint && !b0_paint => {
                self.windows[3] = Some(CardWindow::FaceUp);
                self.phase = 43;
            }
            27 if b0_paint && !b1_paint => {
                self.windows[1] = Some(CardWindow::FaceUp);
                self.phase = 41;
            }
            27 => {
                self.windows[1] = Some(CardWindow::FaceUp);
                self.windows[3] = Some(CardWindow::FaceUp);
                self.phase = 48;
            }
            43 => {
                self.windows[1] = Some(CardWindow::SidePeel);
                self.phase = 31;
            }
            37 => {
                self.windows[1] = Some(CardWindow::FaceUp);
                self.phase = 41;
            }
            41 if b0_paint => {
                self.windows[3] = Some(CardWindow::SidePeel);
                self.phase = 33;
            }
            31 | 33 => {
                self.windows[1] = Some(CardWindow::FaceUp);
                self.windows[3] = Some(CardWindow::FaceUp);
                self.phase = 48;
            }
            41 => {
                self.windows[1] = Some(CardWindow::FaceUp);
                self.windows[3] = Some(CardWindow::FaceUp);
                self.phase = 48;
            }

            // Third card phases
            48 if self.player_third => {
                self.windows[4] = Some(CardWindow::FaceDown);
                self.phase = 114;
            }
            48 if self.banker_third => {
                self.windows[5] = Some(CardWindow::FaceDown);
                self.phase = 115;
            }
            114 if self.forced_third => {
                self.windows[5] = Some(CardWindow::FaceDown);
                self.phase = 115;
            }
            114 if !peel_enabled => {
                self.windows[4] = Some(CardWindow::FaceUp);
                self.phase = 144;
            }
            114 => {
                self.windows[4] = Some(CardWindow::CornerPeel);
                self.phase = 124;
            }
            115 if self.forced_third && !peel_enabled => {
                self.windows[4] = Some(CardWindow::FaceUp);
                self.phase = 144;
            }
            115 if self.forced_third => {
                self.windows[4] = Some(CardWindow::CornerPeel);
                self.phase = 124;
            }
            115 if !peel_enabled => {
                self.windows[5] = Some(CardWindow::FaceUp);
                self.phase = 145;
            }
            115 => {
                self.windows[5] = Some(CardWindow::CornerPeel);
                self.phase = 125;
            }
            124 if peel_enabled && !p2_paint => {
                self.windows[4] = Some(CardWindow::SidePeel);
                self.phase = 134;
            }
            124 => {
                self.windows[4] = Some(CardWindow::FaceUp);
                self.phase = 144;
            }
            134 => {
                self.windows[4] = Some(CardWindow::FaceUp);
                self.phase = 144;
            }
            144 if self.banker_third && !self.forced_third => {
                self.windows[5] = Some(CardWindow::FaceDown);
                self.phase = 115;
            }
            144 if self.banker_third && peel_enabled => {
                self.windows[5] = Some(CardWindow::CornerPeel);
                self.phase = 125;
            }
            144 => {
                self.windows[5] = Some(CardWindow::FaceUp);
                self.phase = 145;
            }
            125 if peel_enabled && !b2_paint => {
                self.windows[5] = Some(CardWindow::SidePeel);
                self.phase = 135;
            }
            125 => {
                self.windows[5] = Some(CardWindow::FaceUp);
                self.phase = 145;
            }
            135 => {
                self.windows[5] = Some(CardWindow::FaceUp);
                self.phase = 145;
            }
            _ => {}
        }
    }
}
