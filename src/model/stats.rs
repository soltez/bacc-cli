use bacc_core::{BaccRound, BaccScoreboard};

pub struct Stats {
    banker: u32,
    player: u32,
    tie: u32,
    banker_pair: u32,
    player_pair: u32,
    natural: u32,
    rounds: u32,
    next_banker: [Option<bool>; 3],
    next_player: [Option<bool>; 3],
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            banker: 0,
            player: 0,
            tie: 0,
            banker_pair: 0,
            player_pair: 0,
            natural: 0,
            rounds: 0,
            next_banker: [None; 3],
            next_player: [None; 3],
        }
    }

    pub fn update(&mut self, round: &BaccRound, scoreboard: &BaccScoreboard) {
        let outcome = round.outcome();
        match outcome.marker() {
            1 => self.player += 1,
            2 => self.banker += 1,
            _ => self.tie += 1,
        }
        if outcome.pairs() & 0x01 != 0 {
            self.player_pair += 1;
        }
        if outcome.pairs() & 0x02 != 0 {
            self.banker_pair += 1;
        }
        if outcome.thirds() == 0 && (outcome.player_value() >= 8 || outcome.banker_value() >= 8) {
            self.natural += 1;
        }
        self.rounds += 1;

        let col_heights = scoreboard.col_heights();
        let last_marker = scoreboard.last_big_road_marker();
        self.next_banker = compute_prediction(col_heights, last_marker, 2);
        self.next_player = compute_prediction(col_heights, last_marker, 1);
    }

    pub fn banker(&self) -> u32 {
        self.banker
    }

    pub fn player(&self) -> u32 {
        self.player
    }

    pub fn tie(&self) -> u32 {
        self.tie
    }

    pub fn banker_pair(&self) -> u32 {
        self.banker_pair
    }

    pub fn player_pair(&self) -> u32 {
        self.player_pair
    }

    pub fn natural(&self) -> u32 {
        self.natural
    }

    pub fn rounds(&self) -> u32 {
        self.rounds
    }

    pub fn next_banker(&self) -> &[Option<bool>; 3] {
        &self.next_banker
    }

    pub fn next_player(&self) -> &[Option<bool>; 3] {
        &self.next_player
    }
}

// Predicts the derived road icon that would appear if the next outcome has next_marker.
// heights: col_heights() from BaccScoreboard ([current, prev, prev-1, prev-2, prev-3]).
// current_marker: last_big_road_marker() (1=player, 2=banker, 0=empty).
// Returns Some(true)=red/trend, Some(false)=blue/chaotic, None=insufficient data.
fn compute_prediction(heights: &[u8], current_marker: u8, next_marker: u8) -> [Option<bool>; 3] {
    let mut result = [None; 3];
    if heights[0] == 0 {
        return result;
    }
    let flips = next_marker != current_marker;
    for (i, &ref_height) in heights[1..].iter().take(3).enumerate() {
        if ref_height > 0 {
            result[i] = Some((heights[0] == ref_height) == flips);
        }
    }
    result
}
