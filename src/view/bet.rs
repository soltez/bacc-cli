use crossterm::{
    cursor, queue,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::io::{self, Write};

use crate::model::bet::{Bet, BetConfig, BetPick};

const COL_BET_L: u16 = 63;
const COL_BET_R: u16 = 79;
const BET_INNER_W: usize = (COL_BET_R - COL_BET_L - 1) as usize; // = 15
const ROW_BET_TOP: u16 = 4;
const ROW_BET_BOT: u16 = 8;

const SECTION_COLORS: [Color; 3] = [Color::DarkBlue, Color::DarkGreen, Color::DarkRed];

pub fn draw_bet_panel(bet: &Bet, out: &mut io::Stdout) -> io::Result<()> {
    draw_top_border(bet, out)?;
    draw_label_row(bet, out)?;
    draw_middle_row(bet, out)?;
    draw_bottom_data_row(bet, out)?;
    draw_bottom_border(out)?;
    queue!(out, ResetColor)
}

fn draw_top_border(bet: &Bet, out: &mut io::Stdout) -> io::Result<()> {
    let balance = bet.format_balance(); // 10 chars, leading space acts as gap after "$"
    let content_w = balance.len() + 5; // "[ $" + balance + " ]"
    let hfill = BET_INNER_W.saturating_sub(content_w);
    let hl = hfill / 2;
    let hr = hfill - hl;

    queue!(
        out,
        cursor::MoveTo(COL_BET_L, ROW_BET_TOP),
        SetBackgroundColor(Color::Black),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2554}{}", "\u{2550}".repeat(hl))?;
    write!(out, "[ $")?;
    queue!(out, SetForegroundColor(Color::White))?;
    write!(out, "{balance}")?;
    queue!(out, SetForegroundColor(Color::DarkGrey))?;
    write!(out, " ]{}\u{2557}", "\u{2550}".repeat(hr))
}

fn draw_label_row(bet: &Bet, out: &mut io::Stdout) -> io::Result<()> {
    let row = ROW_BET_TOP + 1;
    queue!(
        out,
        cursor::MoveTo(COL_BET_L, row),
        SetBackgroundColor(Color::Black),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2551}")?;

    let active_side = bet.input().map(|i| i.pick());
    let sides = [BetPick::Player, BetPick::Tie, BetPick::Banker];
    let labels = ["  P  ", " Tie ", "  B  "];
    let first_letters = ['P', 'T', 'B'];

    for (idx, ((side, label), first)) in sides
        .iter()
        .zip(labels.iter())
        .zip(first_letters.iter())
        .enumerate()
    {
        let bg = SECTION_COLORS[idx];
        queue!(out, SetBackgroundColor(bg))?;

        if active_side == Some(*side) {
            let cfg = side.config();
            queue!(out, SetForegroundColor(Color::White))?;
            write!(out, "{:>3}~ ", cfg.min)?;
        } else {
            let pos = label.find(*first).unwrap_or(0);
            queue!(out, SetForegroundColor(Color::White))?;
            write!(out, "{}", &label[..pos])?;
            queue!(out, SetForegroundColor(Color::Cyan))?;
            write!(out, "{}", first)?;
            queue!(out, SetForegroundColor(Color::White))?;
            write!(out, "{}", &label[pos + first.len_utf8()..])?;
        }
    }

    queue!(
        out,
        SetBackgroundColor(Color::Black),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2551}")
}

fn draw_middle_row(bet: &Bet, out: &mut io::Stdout) -> io::Result<()> {
    let row = ROW_BET_TOP + 2;
    queue!(
        out,
        cursor::MoveTo(COL_BET_L, row),
        SetBackgroundColor(Color::Black),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2551}")?;

    let active_side = bet.input().map(|i| i.pick());
    let show_odds = !bet.any_bet_placed() && active_side.is_none();
    let sides = [BetPick::Player, BetPick::Tie, BetPick::Banker];
    let bets = [bet.player_bet(), bet.tie_bet(), bet.banker_bet()];

    for (idx, side) in sides.iter().enumerate() {
        let bg = SECTION_COLORS[idx];
        let cfg: BetConfig = side.config();
        queue!(
            out,
            SetBackgroundColor(bg),
            SetForegroundColor(Color::White)
        )?;

        if active_side == Some(*side) {
            write!(out, "{:^5}", cfg.max)?;
        } else if bets[idx] > 0 {
            write!(out, "{:>4} ", bets[idx])?;
        } else if show_odds {
            write!(out, "{:^5}", cfg.format_odds())?;
        } else {
            write!(out, "     ")?;
        }
    }

    queue!(
        out,
        SetBackgroundColor(Color::Black),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2551}")
}

fn draw_bottom_data_row(bet: &Bet, out: &mut io::Stdout) -> io::Result<()> {
    let row = ROW_BET_TOP + 3;
    queue!(
        out,
        cursor::MoveTo(COL_BET_L, row),
        SetBackgroundColor(Color::Black),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2551}")?;

    let active_side = bet.input().map(|i| i.pick());
    let sides = [BetPick::Player, BetPick::Tie, BetPick::Banker];

    for (idx, side) in sides.iter().enumerate() {
        let bg = SECTION_COLORS[idx];
        queue!(out, SetBackgroundColor(bg))?;

        if active_side == Some(*side) {
            // input cell: 1 bg space, 3 black digit cells, 1 bg space
            write!(out, " ")?;
            queue!(
                out,
                SetBackgroundColor(Color::Black),
                SetForegroundColor(Color::White)
            )?;
            let digits = bet.input().map(|i| i.digits()).unwrap_or("");
            write!(out, "{:>3}", digits)?;
            queue!(out, SetBackgroundColor(bg))?;
            write!(out, " ")?;
        } else {
            write!(out, "     ")?;
        }
    }

    queue!(
        out,
        SetBackgroundColor(Color::Black),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2551}")
}

fn draw_bottom_border(out: &mut io::Stdout) -> io::Result<()> {
    let clear_label = "[ Clear ]"; // 9 chars
    let cfill = BET_INNER_W.saturating_sub(clear_label.len());
    let cl = cfill / 2;
    let cr = cfill - cl;

    queue!(
        out,
        cursor::MoveTo(COL_BET_L, ROW_BET_BOT),
        SetBackgroundColor(Color::Black),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{255A}{}", "\u{2550}".repeat(cl))?;
    write!(out, "[ ")?;
    queue!(out, SetForegroundColor(Color::Cyan))?;
    write!(out, "C")?;
    queue!(out, SetForegroundColor(Color::White))?;
    write!(out, "lear")?;
    queue!(out, SetForegroundColor(Color::DarkGrey))?;
    write!(out, " ]{}\u{255D}", "\u{2550}".repeat(cr))
}
