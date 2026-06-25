use std::time::SystemTime;

use bacc::BaccShoe;
use bacc_core::BaccRound;
use shoe::{Card, DECK, Shoe as CardShoe};

fn xorshift_shuffle(cards: &mut [Card]) {
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0xDEAD_BEEF_CAFE_BABEu64, |d| d.as_nanos() as u64);
    let mut rng = if seed == 0 {
        0xDEAD_BEEF_CAFE_BABEu64
    } else {
        seed
    };
    for i in (1..cards.len()).rev() {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        let j = (rng as usize) % (i + 1);
        cards.swap(i, j);
    }
}

fn new_shoe() -> BaccShoe {
    let mut deck: Vec<Card> = (0..8).flat_map(|_| DECK).collect();
    for _ in 0..3 {
        xorshift_shuffle(&mut deck);
    }
    // 14-card stub = ~96.6% penetration (14 / 416 cards undealt)
    deck.push(Card::Cut);
    let last = deck.len() - 1;
    deck.swap(14, last);
    BaccShoe::from(CardShoe::from(deck.as_slice()))
}

pub struct Shoe {
    inner: Option<BaccShoe>,
}

impl Shoe {
    pub(super) fn new() -> Self {
        Shoe {
            inner: Some(new_shoe()),
        }
    }

    pub(super) fn next_round(&mut self) -> Option<BaccRound> {
        if let Some(ref mut shoe) = self.inner {
            shoe.next()
        } else {
            None
        }
    }

    pub(super) fn reset(&mut self) {
        self.inner = Some(new_shoe());
    }
}
