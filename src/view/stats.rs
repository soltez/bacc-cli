use crossterm::{
    cursor, queue,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::io::{self, Write};

use crate::model::stats::Stats;

const COL_STATS_L: u16 = 37;
const COL_STATS_R: u16 = 61;
const STATS_INNER_W: usize = (COL_STATS_R - COL_STATS_L - 1) as usize; // = 23
const STATS_DATA_W: usize = STATS_INNER_W - 2; // = 21, after 1 col pad each side
const ROW_STATS_TOP: u16 = 4;
const ROW_STATS_BOT: u16 = 14;

const LABELS: [&str; 9] = [
    "Banker",
    "Player",
    "Tie",
    "Banker Pair",
    "Player Pair",
    "Natural",
    "Round Number",
    "Next Banker",
    "Next Player",
];

const PRED_GLYPHS: [char; 3] = ['\u{25CB}', '\u{25CF}', '\u{2044}'];

pub fn draw_stats_panel(stats: &Stats, out: &mut io::Stdout) -> io::Result<()> {
    let hdr = "Statistics";
    let hfill = STATS_INNER_W.saturating_sub(hdr.len() + 4);
    let hl = hfill / 2;
    let hr = hfill - hl;

    queue!(
        out,
        cursor::MoveTo(COL_STATS_L, ROW_STATS_TOP),
        SetBackgroundColor(Color::Green),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2554}{}", "\u{2550}".repeat(hl))?;
    write!(out, "[ ")?;
    queue!(out, SetForegroundColor(Color::Black))?;
    write!(out, "{hdr}")?;
    queue!(out, SetForegroundColor(Color::DarkGrey))?;
    write!(out, " ]{}\u{2557}", "\u{2550}".repeat(hr))?;

    let counters: [u32; 7] = [
        stats.banker(),
        stats.player(),
        stats.tie(),
        stats.banker_pair(),
        stats.player_pair(),
        stats.natural(),
        stats.rounds(),
    ];

    for (i, label) in LABELS.iter().enumerate() {
        let row = ROW_STATS_TOP + 1 + i as u16;
        queue!(
            out,
            cursor::MoveTo(COL_STATS_L, row),
            SetBackgroundColor(Color::Green),
            SetForegroundColor(Color::DarkGrey)
        )?;
        write!(out, "\u{2551} ")?;
        queue!(out, SetForegroundColor(Color::Black))?;
        write!(out, "{label}")?;

        if i < 7 {
            let value_str = counters[i].to_string();
            let gap = STATS_DATA_W.saturating_sub(label.len() + value_str.len());
            write!(out, "{}", " ".repeat(gap))?;
            queue!(out, SetForegroundColor(Color::DarkGrey))?;
            write!(out, "{value_str} \u{2551}")?;
        } else {
            let predictions = if i == 7 {
                stats.next_banker()
            } else {
                stats.next_player()
            };
            let gap = STATS_DATA_W.saturating_sub(label.len() + PRED_GLYPHS.len());
            write!(out, "{}", " ".repeat(gap))?;
            queue!(out, SetForegroundColor(Color::White))?;
            for (pred, &glyph) in predictions.iter().zip(PRED_GLYPHS.iter()) {
                let bg = match pred {
                    Some(true) => Color::DarkRed,
                    Some(false) => Color::DarkBlue,
                    None => Color::DarkGrey,
                };
                queue!(out, SetBackgroundColor(bg))?;
                let ch = if pred.is_some() { glyph } else { ' ' };
                write!(out, "{ch}")?;
            }
            queue!(
                out,
                SetBackgroundColor(Color::Green),
                SetForegroundColor(Color::DarkGrey)
            )?;
            write!(out, " \u{2551}")?;
        }
    }

    queue!(
        out,
        cursor::MoveTo(COL_STATS_L, ROW_STATS_BOT),
        SetBackgroundColor(Color::Green),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{255A}{}\u{255D}", "\u{2550}".repeat(STATS_INNER_W))?;

    queue!(out, ResetColor)
}
