#[derive(Copy, Clone, PartialEq)]
pub enum CardWindow {
    FaceDown,
    CornerPeel,
    SidePeel,
    FaceUp,
}

impl CardWindow {
    pub fn bounds(&self) -> (usize, usize) {
        match self {
            CardWindow::FaceDown => (0, 0),
            CardWindow::CornerPeel => (2, 2),
            CardWindow::SidePeel => (2, 6),
            CardWindow::FaceUp => (7, 6),
        }
    }
}
