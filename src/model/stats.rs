use bacc::{BaccaratRound, BaccaratScoreboard};

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

    pub fn update(&mut self, round: &BaccaratRound, scoreboard: &BaccaratScoreboard) {
        let bits = round.encode();

        match bits & 0x03 {
            1 => self.player += 1,
            2 => self.banker += 1,
            _ => self.tie += 1,
        }
        if bits & 0x04 != 0 {
            self.player_pair += 1;
        }
        if bits & 0x08 != 0 {
            self.banker_pair += 1;
        }
        if bits & 0x10 == 0
            && bits & 0x20 == 0
            && (bits & 0x0F00 >= 0x0800 || bits & 0xF000 >= 0x8000)
        {
            self.natural += 1;
        }
        self.rounds += 1;

        let cols = parse_big_road_cols(scoreboard);
        self.next_banker = compute_prediction(&cols, 2);
        self.next_player = compute_prediction(&cols, 1);
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

// Extracts (marker, height) for the last 4 big road columns, newest first.
// BigUint column format: byte 0 = row_count n, byte 1 = bead (bits 1-0 = marker),
// bytes 2..2n = remaining row data. Total bytes per column: 1 + 2*n.
fn parse_big_road_cols(scoreboard: &BaccaratScoreboard) -> [Option<(u8, u8)>; 4] {
    let mut cols = [None; 4];
    let bytes = scoreboard.big_road().to_bytes_le();
    let mut pos = 0;
    for slot in &mut cols {
        if pos >= bytes.len() {
            break;
        }
        let n = bytes[pos];
        if n == 0 {
            break;
        }
        let marker = bytes[pos + 1] & 0x03;
        *slot = Some((marker, n));
        pos += 1 + 2 * usize::from(n);
    }
    cols
}

// Computes the next icon for each derived road if the next outcome has the given marker.
// cols[0] = current column, cols[i+1] = reference column for road i (BEB=1, SR=2, CP=3).
// Red icon = Some(true) = trending. Blue icon = Some(false) = chaotic. None = insufficient data.
// Rule: Red when (current_height == ref_height) == (next outcome flips the column side).
fn compute_prediction(cols: &[Option<(u8, u8)>; 4], next_marker: u8) -> [Option<bool>; 3] {
    let mut result = [None; 3];
    let Some((current_marker, current_height)) = cols[0] else {
        return result;
    };
    let flips = next_marker != current_marker;
    for (i, slot) in result.iter_mut().enumerate() {
        if let Some((_, ref_height)) = cols[i + 1] {
            *slot = Some((current_height == ref_height) == flips);
        }
    }
    result
}
