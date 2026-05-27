use crossterm::{
    cursor, execute, queue,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};

use crate::model::display_options::DisplayOptions;
use crate::model::game::Game;
use crate::model::round_state::RoundState;
use crate::view::bet::draw_bet_panel;
use crate::view::cards::draw_card;
use crate::view::scoreboard::{
    draw_bead_plate, draw_big_road, draw_derived_roads, draw_scoreboard_box,
};
use crate::view::stats::draw_stats_panel;

// Terminal dimensions
const TERM_ROWS: u16 = 24;
//const TERM_COLS: u16 = 80;

// Horizontal divider
const COL_DIVIDER: u16 = 36;

// Title panel
const COL_TITLE_R: u16 = 79;
const TITLE_W: usize = (COL_TITLE_R - COL_PLAYER_BOX_L + 1) as usize; // = 43
const TITLE_INNER_W: usize = TITLE_W - 2; // = 41
const ROW_TITLE_TOP: u16 = 0;
const CREDIT: &str = concat!(
    "Version ",
    env!("CARGO_PKG_VERSION"),
    "  (C)2026     soltez.com"
);
const _: () = assert!(CREDIT.len() == TITLE_INNER_W - 4);

// Hand box rows
const ROW_HAND_BOX_TOP: u16 = 15;
const ROW_HAND_BOX_BOT: u16 = ROW_HAND_BOX_TOP + 1; // = 16

// Hand box columns
const HAND_BOX_W: u16 = 21;
const HAND_BOX_INNER_W: usize = (HAND_BOX_W - 2) as usize; // = 19
const COL_PLAYER_BOX_L: u16 = COL_DIVIDER + 1; // = 37
const COL_PLAYER_BOX_R: u16 = COL_PLAYER_BOX_L + HAND_BOX_W - 1; // = 57
const COL_BANKER_BOX_L: u16 = COL_PLAYER_BOX_R + 2; // = 59 (1 col gap)
//const COL_BANKER_BOX_R: u16 = TERM_COLS - 1;                      // = 79

// Card layout
pub(crate) const CARD_W: u16 = 7;
pub(crate) const CARD_H: u16 = 6;
const ROW_CARD_TOP: u16 = ROW_HAND_BOX_BOT + 1; // = 17
const ROW_CARD_BOT: u16 = ROW_CARD_TOP + CARD_H - 1; // = 22
const ROW_SCORE: u16 = TERM_ROWS - 1; // = 23

fn draw_title_panel(out: &mut io::Stdout) -> io::Result<()> {
    queue!(
        out,
        cursor::MoveTo(COL_PLAYER_BOX_L, ROW_TITLE_TOP),
        SetBackgroundColor(Color::DarkMagenta),
        SetForegroundColor(Color::White)
    )?;
    write!(out, "\u{2554}{}", "\u{2550}".repeat(11))?;
    write!(out, "[ ")?;
    queue!(out, SetForegroundColor(Color::Yellow))?;
    write!(out, "B A C C A R A T")?;
    queue!(out, SetForegroundColor(Color::White))?;
    write!(out, " ]{}\u{2557}", "\u{2550}".repeat(11))?;

    queue!(
        out,
        cursor::MoveTo(COL_PLAYER_BOX_L, ROW_TITLE_TOP + 1),
        SetForegroundColor(Color::White)
    )?;
    write!(out, "\u{2551}")?;
    queue!(out, SetForegroundColor(Color::Yellow))?;
    write!(out, "  {}  ", CREDIT)?;
    queue!(out, SetForegroundColor(Color::White))?;
    write!(out, "\u{2551}")?;

    queue!(out, cursor::MoveTo(COL_PLAYER_BOX_L, ROW_TITLE_TOP + 2),)?;
    write!(out, "\u{255A}{}\u{255D}", "\u{2550}".repeat(TITLE_INNER_W))?;

    queue!(out, ResetColor)
}

fn draw_backgrounds(out: &mut io::Stdout) -> io::Result<()> {
    for row in 0..TERM_ROWS {
        queue!(
            out,
            cursor::MoveTo(0, row),
            SetBackgroundColor(Color::DarkGrey)
        )?;
        write!(out, "{}", " ".repeat(COL_DIVIDER as usize))?;
    }
    for row in ROW_CARD_TOP..=ROW_CARD_BOT {
        for col in [COL_PLAYER_BOX_L, COL_BANKER_BOX_L] {
            queue!(
                out,
                cursor::MoveTo(col, row),
                SetBackgroundColor(Color::White)
            )?;
            write!(out, "{}", " ".repeat(HAND_BOX_W as usize))?;
        }
    }
    queue!(out, ResetColor)
}

fn draw_hand_boxes(out: &mut io::Stdout) -> io::Result<()> {
    let boxes = [
        (COL_BANKER_BOX_L, "Banker's Hand", Color::DarkRed),
        (COL_PLAYER_BOX_L, "Player's Hand", Color::DarkBlue),
    ];
    for (box_l, label, label_color) in boxes {
        let content_w = label.len() + 4; // label + '[ ' + ' ]'
        let hfill = HAND_BOX_INNER_W.saturating_sub(content_w);
        let hl = hfill / 2;
        let hr = hfill - hl;

        queue!(
            out,
            cursor::MoveTo(box_l, ROW_HAND_BOX_TOP),
            SetBackgroundColor(label_color),
            SetForegroundColor(Color::White)
        )?;
        write!(out, "\u{2554}{}[ ", "\u{2550}".repeat(hl))?;
        queue!(out, SetForegroundColor(Color::Yellow))?;
        write!(out, "{label}")?;
        queue!(out, SetForegroundColor(Color::White))?;
        write!(out, " ]{}\u{2557}", "\u{2550}".repeat(hr))?;

        queue!(
            out,
            cursor::MoveTo(box_l, ROW_HAND_BOX_BOT),
            SetBackgroundColor(label_color),
            SetForegroundColor(Color::White)
        )?;
        write!(
            out,
            "\u{255A}{}\u{255D}",
            "\u{2550}".repeat(HAND_BOX_INNER_W)
        )?;
    }
    Ok(())
}

fn draw_card_panels(
    round: &RoundState,
    display: &DisplayOptions,
    out: &mut io::Stdout,
) -> io::Result<()> {
    if !display.show_hands() || round.phase() == 0 {
        return Ok(());
    }

    for (slot_idx, window) in round.windows().iter().enumerate() {
        if let Some(window) = window {
            let (box_l, cards) = if slot_idx % 2 == 0 {
                (COL_PLAYER_BOX_L, round.player_cards())
            } else {
                (COL_BANKER_BOX_L, round.banker_cards())
            };
            let card_idx = slot_idx / 2;
            let col = box_l + card_idx as u16 * CARD_W;
            if let Some(&card) = cards.get(card_idx) {
                draw_card(out, card, col, ROW_CARD_TOP, *window)?;
            }
        }
    }

    let score_w: u16 = 10;
    let pad = (HAND_BOX_W - score_w) / 2;
    if round.has_player_result() {
        queue!(
            out,
            cursor::MoveTo(COL_PLAYER_BOX_L + pad, ROW_SCORE),
            SetBackgroundColor(Color::Red),
            SetForegroundColor(Color::White)
        )?;
        write!(out, " Score {:>2} ", round.player_score())?;
    }
    if round.has_banker_result() {
        queue!(
            out,
            cursor::MoveTo(COL_BANKER_BOX_L + pad, ROW_SCORE),
            SetBackgroundColor(Color::Red),
            SetForegroundColor(Color::White)
        )?;
        write!(out, " Score {:>2} ", round.banker_score())?;
    }

    Ok(())
}

pub fn render(game: &Game, out: &mut io::Stdout) -> io::Result<()> {
    execute!(out, terminal::Clear(ClearType::All), cursor::Hide)?;
    draw_backgrounds(out)?;
    draw_title_panel(out)?;
    draw_stats_panel(game.stats(), out)?;
    draw_bet_panel(game.bet(), out)?;
    draw_scoreboard_box(game.shoe_number(), out)?;
    draw_hand_boxes(out)?;
    draw_card_panels(game.round(), game.display(), out)?;
    draw_bead_plate(game.scoreboard_cache(), out)?;
    draw_big_road(game.scoreboard_cache(), out)?;
    draw_derived_roads(game.scoreboard_cache(), out)?;
    queue!(out, ResetColor)?;
    out.flush()
}
