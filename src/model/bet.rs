#[derive(Clone, Copy, PartialEq)]
pub enum BetPick {
    Player,
    Tie,
    Banker,
}

impl BetPick {
    pub fn min(self) -> u32 {
        match self {
            BetPick::Player => 200,
            BetPick::Tie => 25,
            BetPick::Banker => 200,
        }
    }

    pub fn max(self) -> u32 {
        match self {
            BetPick::Player => 800,
            BetPick::Tie => 100,
            BetPick::Banker => 800,
        }
    }

    pub fn odds_num(self) -> u32 {
        match self {
            BetPick::Player => 1,
            BetPick::Tie => 8,
            BetPick::Banker => 19,
        }
    }

    pub fn odds_den(self) -> u32 {
        match self {
            BetPick::Player => 1,
            BetPick::Tie => 1,
            BetPick::Banker => 20,
        }
    }

    pub fn format_odds(self) -> String {
        format!("{}:{}", self.odds_num(), self.odds_den())
    }
}

pub struct BetInput {
    pick: BetPick,
    digits: String,
}

impl BetInput {
    fn new(pick: BetPick) -> Self {
        BetInput {
            pick,
            digits: String::new(),
        }
    }

    pub fn pick(&self) -> BetPick {
        self.pick
    }

    pub fn digits(&self) -> &str {
        &self.digits
    }

    fn push_digit(&mut self, d: char) {
        if self.digits.len() < 3 {
            self.digits.push(d);
        }
    }

    fn pop_digit(&mut self) {
        self.digits.pop();
    }

    fn parse_amount(&self) -> Option<u32> {
        self.digits.parse::<u32>().ok()
    }
}

pub struct Bet {
    balance: u32,
    player_bet: u32,
    tie_bet: u32,
    banker_bet: u32,
    input: Option<BetInput>,
}

const INITIAL_BALANCE_CENTS: u32 = 1_000_000;

impl Bet {
    pub fn new() -> Self {
        Bet {
            balance: INITIAL_BALANCE_CENTS,
            player_bet: 0,
            tie_bet: 0,
            banker_bet: 0,
            input: None,
        }
    }

    pub fn player_bet(&self) -> u32 {
        self.player_bet
    }

    pub fn tie_bet(&self) -> u32 {
        self.tie_bet
    }

    pub fn banker_bet(&self) -> u32 {
        self.banker_bet
    }

    pub fn input(&self) -> Option<&BetInput> {
        self.input.as_ref()
    }

    pub fn any_bet_placed(&self) -> bool {
        self.player_bet > 0 || self.tie_bet > 0 || self.banker_bet > 0
    }

    pub fn start_input(&mut self, pick: BetPick) {
        self.input = Some(BetInput::new(pick));
    }

    pub fn push_digit(&mut self, d: char) {
        if let Some(ref mut inp) = self.input {
            inp.push_digit(d);
        }
    }

    pub fn pop_digit(&mut self) {
        if let Some(ref mut inp) = self.input {
            inp.pop_digit();
        }
    }

    pub fn confirm(&mut self) -> bool {
        let Some(ref inp) = self.input else {
            return false;
        };
        let Some(amount) = inp.parse_amount() else {
            return false;
        };
        let pick = inp.pick;
        if !(pick.min()..=pick.max()).contains(&amount) {
            return false;
        }
        match pick {
            BetPick::Player => {
                self.balance = self.balance.saturating_add(self.player_bet * 100);
                self.balance = self.balance.saturating_add(self.banker_bet * 100);
                self.player_bet = amount;
                self.banker_bet = 0;
            }
            BetPick::Banker => {
                self.balance = self.balance.saturating_add(self.banker_bet * 100);
                self.balance = self.balance.saturating_add(self.player_bet * 100);
                self.banker_bet = amount;
                self.player_bet = 0;
            }
            BetPick::Tie => {
                self.balance = self.balance.saturating_add(self.tie_bet * 100);
                self.tie_bet = amount;
            }
        }
        self.balance = self.balance.saturating_sub(amount * 100);
        self.input = None;
        true
    }

    pub fn cancel_input(&mut self) {
        self.input = None;
    }

    pub fn clear(&mut self) {
        self.balance = self
            .balance
            .saturating_add((self.player_bet + self.tie_bet + self.banker_bet) * 100);
        self.player_bet = 0;
        self.tie_bet = 0;
        self.banker_bet = 0;
        self.input = None;
    }

    /// Settle bets after a round. marker: 1=player, 2=banker, other=tie.
    /// Deducts all wagered amounts then adds back stake+profit for the winning side.
    pub fn settle(&mut self, marker: u8) {
        match marker {
            1 => {
                self.balance = self.balance.saturating_add(
                    self.player_bet * BetPick::Player.odds_num() / BetPick::Player.odds_den() * 100,
                );
                self.balance = self.balance.saturating_sub(self.banker_bet * 100);
            }
            2 => {
                self.balance = self.balance.saturating_add(
                    self.banker_bet * BetPick::Banker.odds_num() / BetPick::Banker.odds_den() * 100,
                );
                self.balance = self.balance.saturating_sub(self.player_bet * 100);
            }
            _ => {
                self.balance = self.balance.saturating_add(
                    self.tie_bet * (BetPick::Tie.odds_num() + BetPick::Tie.odds_den())
                        / BetPick::Tie.odds_den()
                        * 100,
                );
            }
        }
        self.tie_bet = 0;
    }

    /// Returns a 10-char right-aligned balance string, e.g. " 10,000.00".
    pub fn format_balance(&self) -> String {
        let dollars = self.balance / 100;
        let cents = self.balance % 100;
        let dollar_str = format_with_commas(dollars);
        let raw = format!("{}.{:02}", dollar_str, cents);
        format!("{:>10}", raw)
    }
}

fn format_with_commas(n: u32) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}
