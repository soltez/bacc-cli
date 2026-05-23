pub struct DisplayOptions {
    show_hands: bool,
    deal_speed_ms: u64,
    peel_enabled: bool,
}

impl DisplayOptions {
    pub fn new() -> Self {
        DisplayOptions {
            show_hands: true,
            deal_speed_ms: 50,
            peel_enabled: false,
        }
    }

    pub fn show_hands(&self) -> bool {
        self.show_hands
    }

    pub fn deal_speed_ms(&self) -> u64 {
        self.deal_speed_ms
    }

    pub fn peel_enabled(&self) -> bool {
        self.peel_enabled
    }

    pub fn toggle_show_hands(&mut self) {
        self.show_hands = !self.show_hands;
    }

    pub fn toggle_peel_enabled(&mut self) {
        self.peel_enabled = !self.peel_enabled;
    }
}
