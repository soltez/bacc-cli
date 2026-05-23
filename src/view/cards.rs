use crossterm::{
    cursor, queue,
    style::{Color, SetBackgroundColor, SetForegroundColor},
};
use kev::{CardInt, Rank, Suit};
use std::io::{self, Write};

use Cell::{Blc, Blk, Brc, Hrz, Pip, Rnk, Sut, Tlc, Trc, Vrt};

#[derive(Copy, Clone)]
enum Cell {
    Tlc,
    Trc,
    Blc,
    Brc,
    Hrz,
    Vrt,
    Blk,
    Pip,
    Rnk,
    Sut,
}

use crate::model::card_window::CardWindow;

struct CardTemplate {
    cells: [[Cell; 7]; 6],
    double_border: bool,
}

static TMPL_NO_SIDE: CardTemplate = CardTemplate {
    double_border: false,
    cells: [
        [Tlc, Hrz, Hrz, Hrz, Hrz, Hrz, Trc],
        [Vrt, Blk, Blk, Blk, Blk, Blk, Vrt],
        [Vrt, Blk, Blk, Rnk, Blk, Blk, Vrt],
        [Vrt, Blk, Blk, Sut, Blk, Blk, Vrt],
        [Vrt, Blk, Blk, Blk, Blk, Blk, Vrt],
        [Blc, Hrz, Hrz, Hrz, Hrz, Hrz, Brc],
    ],
};

static TMPL_PAINT: CardTemplate = CardTemplate {
    double_border: true,
    cells: [
        [Tlc, Hrz, Hrz, Hrz, Hrz, Hrz, Trc],
        [Vrt, Blk, Blk, Blk, Blk, Blk, Vrt],
        [Vrt, Blk, Blk, Rnk, Blk, Blk, Vrt],
        [Vrt, Blk, Blk, Sut, Blk, Blk, Vrt],
        [Vrt, Blk, Blk, Blk, Blk, Blk, Vrt],
        [Blc, Hrz, Hrz, Hrz, Hrz, Hrz, Brc],
    ],
};

static TMPL_TWO_SIDE: CardTemplate = CardTemplate {
    double_border: false,
    cells: [
        [Tlc, Hrz, Hrz, Hrz, Hrz, Hrz, Trc],
        [Vrt, Pip, Blk, Blk, Blk, Blk, Vrt],
        [Vrt, Pip, Blk, Blk, Rnk, Blk, Vrt],
        [Vrt, Blk, Blk, Blk, Sut, Blk, Vrt],
        [Vrt, Blk, Blk, Blk, Blk, Blk, Vrt],
        [Blc, Hrz, Hrz, Hrz, Hrz, Hrz, Brc],
    ],
};

static TMPL_THREE_SIDE: CardTemplate = CardTemplate {
    double_border: false,
    cells: [
        [Tlc, Hrz, Hrz, Hrz, Hrz, Hrz, Trc],
        [Vrt, Pip, Blk, Blk, Blk, Blk, Vrt],
        [Vrt, Pip, Blk, Blk, Rnk, Blk, Vrt],
        [Vrt, Pip, Blk, Blk, Sut, Blk, Vrt],
        [Vrt, Blk, Blk, Blk, Blk, Blk, Vrt],
        [Blc, Hrz, Hrz, Hrz, Hrz, Hrz, Brc],
    ],
};

static TMPL_FOUR_SIDE: CardTemplate = CardTemplate {
    double_border: false,
    cells: [
        [Tlc, Hrz, Hrz, Hrz, Hrz, Hrz, Trc],
        [Vrt, Pip, Blk, Blk, Blk, Blk, Vrt],
        [Vrt, Pip, Blk, Blk, Rnk, Blk, Vrt],
        [Vrt, Pip, Blk, Blk, Sut, Blk, Vrt],
        [Vrt, Pip, Blk, Blk, Blk, Blk, Vrt],
        [Blc, Hrz, Hrz, Hrz, Hrz, Hrz, Brc],
    ],
};

fn template_for(rank: Rank) -> &'static CardTemplate {
    match rank {
        Rank::Jack | Rank::Queen | Rank::King => &TMPL_PAINT,
        Rank::Four | Rank::Five => &TMPL_TWO_SIDE,
        Rank::Six | Rank::Seven => &TMPL_THREE_SIDE,
        Rank::Eight | Rank::Nine | Rank::Ten => &TMPL_FOUR_SIDE,
        _ => &TMPL_NO_SIDE,
    }
}

fn suit_symbol(suit: Suit) -> char {
    match suit {
        Suit::Spade => '\u{2660}',
        Suit::Heart => '\u{2665}',
        Suit::Diamond => '\u{2666}',
        Suit::Club => '\u{2663}',
    }
}

fn suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Heart | Suit::Diamond => Color::DarkRed,
        Suit::Spade | Suit::Club => Color::Black,
    }
}

fn rank_str(rank: Rank) -> &'static str {
    match rank {
        Rank::Ace => "A",
        Rank::Deuce => "2",
        Rank::Trey => "3",
        Rank::Four => "4",
        Rank::Five => "5",
        Rank::Six => "6",
        Rank::Seven => "7",
        Rank::Eight => "8",
        Rank::Nine => "9",
        Rank::Ten => "T",
        Rank::Jack => "J",
        Rank::Queen => "Q",
        Rank::King => "K",
    }
}

/// Draw a single card occupying 7 cols x 6 rows starting at (col, row).
/// The window parameter controls how much of the face is revealed.
pub(super) fn draw_card(
    out: &mut io::Stdout,
    card: CardInt,
    col: u16,
    row: u16,
    window: CardWindow,
) -> io::Result<()> {
    let rank = rank_str(card.rank());
    let suit_ch = suit_symbol(card.suit());
    let fg = suit_color(card.suit());
    let bg = Color::White;
    let border_fg = Color::Black;

    let tmpl = template_for(card.rank());
    let (win_cols, win_rows) = window.bounds();

    for (r, row_cells) in tmpl.cells.iter().enumerate() {
        for (c, cell) in row_cells.iter().enumerate() {
            queue!(
                out,
                cursor::MoveTo(col + c as u16, row + r as u16),
                SetBackgroundColor(bg)
            )?;

            let revealed = r < win_rows && c < win_cols;

            match cell {
                Tlc => {
                    queue!(out, SetForegroundColor(border_fg))?;
                    if revealed && tmpl.double_border {
                        write!(out, "\u{2554}")?
                    } else {
                        write!(out, "\u{250C}")?
                    };
                }
                Trc => {
                    queue!(out, SetForegroundColor(border_fg))?;
                    if revealed && tmpl.double_border {
                        write!(out, "\u{2557}")?
                    } else {
                        write!(out, "\u{2510}")?
                    };
                }
                Blc => {
                    queue!(out, SetForegroundColor(border_fg))?;
                    if revealed && tmpl.double_border {
                        write!(out, "\u{255A}")?
                    } else {
                        write!(out, "\u{2514}")?
                    };
                }
                Brc => {
                    queue!(out, SetForegroundColor(border_fg))?;
                    if revealed && tmpl.double_border {
                        write!(out, "\u{255D}")?
                    } else {
                        write!(out, "\u{2518}")?
                    };
                }
                Hrz => {
                    queue!(out, SetForegroundColor(border_fg))?;
                    if revealed && tmpl.double_border {
                        write!(out, "\u{2550}")?
                    } else {
                        write!(out, "\u{2500}")?
                    };
                }
                Vrt => {
                    queue!(out, SetForegroundColor(border_fg))?;
                    if revealed && tmpl.double_border {
                        write!(out, "\u{2551}")?
                    } else {
                        write!(out, "\u{2502}")?
                    };
                }
                Blk => {
                    if revealed {
                        write!(out, " ")?;
                    } else {
                        queue!(out, SetForegroundColor(Color::Grey))?;
                        write!(out, "\u{2592}")?;
                    }
                }
                Pip => {
                    if revealed {
                        queue!(out, SetForegroundColor(fg))?;
                        write!(out, "{suit_ch}")?;
                    } else {
                        queue!(out, SetForegroundColor(Color::Grey))?;
                        write!(out, "\u{2592}")?;
                    }
                }
                Rnk => {
                    if revealed {
                        queue!(out, SetForegroundColor(fg))?;
                        write!(out, "{rank}")?;
                    } else {
                        queue!(out, SetForegroundColor(Color::Grey))?;
                        write!(out, "\u{2592}")?;
                    }
                }
                Sut => {
                    if revealed {
                        queue!(out, SetForegroundColor(fg))?;
                        write!(out, "{suit_ch}")?;
                    } else {
                        queue!(out, SetForegroundColor(Color::Grey))?;
                        write!(out, "\u{2592}")?;
                    }
                }
            }
        }
    }

    Ok(())
}
