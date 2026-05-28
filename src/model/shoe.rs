use bacc::{BaccaratRound, BaccaratShoe};

fn new_shoe() -> BaccaratShoe {
    BaccaratShoe::new(8, 3, 0.965)
}

pub struct Shoe {
    inner: Option<BaccaratShoe>,
}

impl Shoe {
    pub(super) fn new() -> Self {
        Shoe {
            inner: Some(new_shoe()),
        }
    }

    pub(super) fn next_round(&mut self) -> Option<BaccaratRound> {
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
