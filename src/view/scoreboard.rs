use crossterm::{
    cursor, queue,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::io::{self, Write};

use bacc_core::BaccScoreboard;

// Scoreboard panel
const SCORE_COL_L: u16 = 0;
const SCORE_COL_R: u16 = 35;
const SCORE_COL_INNER_L: u16 = SCORE_COL_L + 1;
const SCORE_COL_INNER_R: u16 = SCORE_COL_R - 1;
const SCORE_INNER_W: usize = (SCORE_COL_R - SCORE_COL_L - 1) as usize;
const ROW_SCORE_TOP: u16 = 0;
const ROW_SCORE_BOT: u16 = 23;

// Bead plate
const BEAD_ROWS: usize = 3;
const BEAD_COLS: usize = (SCORE_COL_INNER_R - SCORE_COL_INNER_L - 1) as usize;
const ROW_BEAD_TOP: u16 = 1;
const ROW_BEAD_DATA: u16 = ROW_BEAD_TOP + 1;
const ROW_BEAD_BOT: u16 = ROW_BEAD_DATA + BEAD_ROWS as u16;

// Big road
const BIG_ROAD_ROWS: usize = 6;
const BIG_ROAD_COLS: usize = BEAD_COLS;
const ROW_BIG_ROAD_TOP: u16 = ROW_BEAD_BOT + 1;
const ROW_BIG_ROAD_DATA: u16 = ROW_BIG_ROAD_TOP + 1;
const ROW_BIG_ROAD_BOT: u16 = ROW_BIG_ROAD_DATA + BIG_ROAD_ROWS as u16;

// Derived roads
const ROW_BEB_TOP: u16 = ROW_BIG_ROAD_BOT + 1;
const ROW_BEB_DATA: u16 = ROW_BEB_TOP + 1;
const ROW_BEB_BOT: u16 = ROW_BEB_DATA + 1;
const ROW_SR_TOP: u16 = ROW_BEB_BOT + 1;
const ROW_SR_DATA: u16 = ROW_SR_TOP + 1;
const ROW_SR_BOT: u16 = ROW_SR_DATA + 1;
const ROW_CP_TOP: u16 = ROW_SR_BOT + 1;
const ROW_CP_DATA: u16 = ROW_CP_TOP + 1;
const ROW_CP_BOT: u16 = ROW_CP_DATA + 1;

fn outcome_color(outcome: u8) -> Color {
    match outcome {
        1 => Color::DarkBlue,
        2 => Color::DarkRed,
        3 => Color::DarkGreen,
        _ => Color::Reset,
    }
}

pub fn draw_scoreboard_box(shoe_number: u32, out: &mut io::Stdout) -> io::Result<()> {
    let label = format!("[ Score Card   Shoe #{shoe_number:>3} ]");
    let hfill = SCORE_INNER_W.saturating_sub(label.len());
    let hl = hfill / 2;
    let hr = hfill - hl;

    queue!(
        out,
        cursor::MoveTo(SCORE_COL_L, ROW_SCORE_TOP),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2554}{}", "\u{2550}".repeat(hl))?;
    write!(out, "{label}")?;
    write!(out, "{}\u{2557}", "\u{2550}".repeat(hr))?;

    for row in (ROW_SCORE_TOP + 1)..ROW_SCORE_BOT {
        queue!(
            out,
            cursor::MoveTo(SCORE_COL_L, row),
            SetForegroundColor(Color::White),
            SetBackgroundColor(Color::DarkGrey)
        )?;
        write!(out, "\u{2551}")?;
        queue!(out, cursor::MoveTo(SCORE_COL_R, row))?;
        write!(out, "\u{2551}")?;
    }

    queue!(
        out,
        cursor::MoveTo(SCORE_COL_L, ROW_SCORE_BOT),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{255A}{}\u{255D}", "\u{2550}".repeat(SCORE_INNER_W))?;

    queue!(out, ResetColor)
}

pub fn draw_bead_plate(scoreboard: &BaccScoreboard, out: &mut io::Stdout) -> io::Result<()> {
    let bp_hdr = "[ Bead Plate ]";
    let bp_hfill = BEAD_COLS.saturating_sub(bp_hdr.len());
    let bp_hl = bp_hfill / 2;
    let bp_hr = bp_hfill - bp_hl;

    queue!(
        out,
        cursor::MoveTo(SCORE_COL_INNER_L, ROW_BEAD_TOP),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2554}{}", "\u{2550}".repeat(bp_hl))?;
    write!(out, "{bp_hdr}")?;
    write!(out, "{}\u{2557}", "\u{2550}".repeat(bp_hr))?;

    for r in 0..BEAD_ROWS {
        let row = ROW_BEAD_DATA + r as u16;
        queue!(
            out,
            cursor::MoveTo(SCORE_COL_INNER_L, row),
            SetForegroundColor(Color::White),
            SetBackgroundColor(Color::DarkGrey)
        )?;
        write!(out, "\u{2551}")?;
        for c in 0..BEAD_COLS {
            queue!(
                out,
                cursor::MoveTo(SCORE_COL_INNER_L + 1 + c as u16, row),
                SetBackgroundColor(Color::DarkGrey)
            )?;
            write!(out, " ")?;
        }
        queue!(
            out,
            cursor::MoveTo(SCORE_COL_INNER_R, row),
            SetForegroundColor(Color::White),
            SetBackgroundColor(Color::DarkGrey)
        )?;
        write!(out, "\u{2551}")?;
    }

    queue!(
        out,
        cursor::MoveTo(SCORE_COL_INNER_L, ROW_BEAD_BOT),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{255A}{}\u{255D}", "\u{2550}".repeat(BEAD_COLS))?;

    // simulate_bead_plate uses ROWS=6; re-map entries to our BEAD_ROWS=3 display.
    let grid = scoreboard.simulate_bead_plate(BEAD_COLS * 2);
    let mut entry_idx = 0usize;
    'outer: for column in grid.iter() {
        for &(bead_byte, aux_byte) in column.iter() {
            if bead_byte == 0 {
                continue;
            }
            let display_col = entry_idx / BEAD_ROWS;
            let display_row = entry_idx % BEAD_ROWS;
            if display_col >= BEAD_COLS {
                break 'outer;
            }
            let outcome = bead_byte & 0x03;
            let hand_val = aux_byte & 0x0F;
            queue!(
                out,
                cursor::MoveTo(
                    SCORE_COL_INNER_L + 1 + display_col as u16,
                    ROW_BEAD_DATA + display_row as u16
                ),
                SetBackgroundColor(outcome_color(outcome)),
                SetForegroundColor(Color::White)
            )?;
            write!(out, "{hand_val}")?;
            entry_idx += 1;
        }
    }

    queue!(out, ResetColor)
}

pub fn draw_big_road(scoreboard: &BaccScoreboard, out: &mut io::Stdout) -> io::Result<()> {
    let br_hdr = "[ Big Road ]";
    let br_hfill = BEAD_COLS.saturating_sub(br_hdr.len());
    let br_hl = br_hfill / 2;
    let br_hr = br_hfill - br_hl;

    queue!(
        out,
        cursor::MoveTo(SCORE_COL_INNER_L, ROW_BIG_ROAD_TOP),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2554}{}", "\u{2550}".repeat(br_hl))?;
    write!(out, "{br_hdr}")?;
    write!(out, "{}\u{2557}", "\u{2550}".repeat(br_hr))?;

    for row in ROW_BIG_ROAD_DATA..ROW_BIG_ROAD_BOT {
        queue!(
            out,
            cursor::MoveTo(SCORE_COL_INNER_L, row),
            SetForegroundColor(Color::White),
            SetBackgroundColor(Color::DarkGrey)
        )?;
        write!(out, "\u{2551}")?;
        queue!(out, cursor::MoveTo(SCORE_COL_INNER_R, row))?;
        write!(out, "\u{2551}")?;
    }

    queue!(
        out,
        cursor::MoveTo(SCORE_COL_INNER_L, ROW_BIG_ROAD_BOT),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{255A}{}\u{255D}", "\u{2550}".repeat(BEAD_COLS))?;

    let grid = scoreboard.simulate_big_road();

    for (col, column) in grid.iter().enumerate().take(BIG_ROAD_COLS) {
        for (row, &(bead_byte, _)) in column.iter().enumerate().take(BIG_ROAD_ROWS) {
            if bead_byte == 0 {
                continue;
            }
            let outcome = bead_byte & 0x03;
            queue!(
                out,
                cursor::MoveTo(
                    SCORE_COL_INNER_L + 1 + col as u16,
                    ROW_BIG_ROAD_DATA + row as u16
                ),
                SetBackgroundColor(outcome_color(outcome)),
                SetForegroundColor(Color::White)
            )?;
            write!(out, "\u{25CB}")?;
        }
    }

    queue!(out, ResetColor)
}

fn draw_derived_road(
    label: &str,
    entries: &[(u8, u8)],
    row_top: u16,
    row_data: u16,
    row_bot: u16,
    out: &mut io::Stdout,
) -> io::Result<()> {
    let hfill = BEAD_COLS.saturating_sub(label.len());
    let hl = hfill / 2;
    let hr = hfill - hl;

    queue!(
        out,
        cursor::MoveTo(SCORE_COL_INNER_L, row_top),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2554}{}", "\u{2550}".repeat(hl))?;
    write!(out, "{label}")?;
    write!(out, "{}\u{2557}", "\u{2550}".repeat(hr))?;

    queue!(
        out,
        cursor::MoveTo(SCORE_COL_INNER_L, row_data),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2551}")?;
    queue!(out, cursor::MoveTo(SCORE_COL_INNER_R, row_data))?;
    write!(out, "\u{2551}")?;

    queue!(
        out,
        cursor::MoveTo(SCORE_COL_INNER_L, row_bot),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{255A}{}\u{255D}", "\u{2550}".repeat(BEAD_COLS))?;

    let visible = if entries.len() > BEAD_COLS {
        &entries[entries.len() - BEAD_COLS..]
    } else {
        entries
    };
    for (i, &(count, marker)) in visible.iter().enumerate() {
        let bg = outcome_color(marker);
        let digit = char::from_digit(count as u32, 10).unwrap_or('?');
        queue!(
            out,
            cursor::MoveTo(SCORE_COL_INNER_L + 1 + i as u16, row_data),
            SetBackgroundColor(bg),
            SetForegroundColor(Color::White)
        )?;
        write!(out, "{digit}")?;
    }

    Ok(())
}

fn flatten_derived_grid(grid: &[[(u8, u8); 6]]) -> Vec<(u8, u8)> {
    let mut result: Vec<(u8, u8)> = Vec::new();
    for col in grid.iter() {
        let count = col.iter().filter(|&&(b, _)| b != 0).count() as u8;
        if count == 0 {
            continue;
        }
        let icon = col
            .iter()
            .find(|&&(b, _)| b != 0)
            .map(|&(b, _)| b & 0x03)
            .unwrap_or(0);
        if let Some(last) = result.last_mut()
            && last.1 == icon
        {
            last.0 += count;
            continue;
        }
        result.push((count, icon));
    }
    result
}

pub fn draw_derived_roads(scoreboard: &BaccScoreboard, out: &mut io::Stdout) -> io::Result<()> {
    let beb = flatten_derived_grid(&scoreboard.simulate_derived_road(0));
    draw_derived_road(
        "[ Big Eye Boy ]",
        &beb,
        ROW_BEB_TOP,
        ROW_BEB_DATA,
        ROW_BEB_BOT,
        out,
    )?;

    let sr = flatten_derived_grid(&scoreboard.simulate_derived_road(1));
    draw_derived_road(
        "[ Small Road ]",
        &sr,
        ROW_SR_TOP,
        ROW_SR_DATA,
        ROW_SR_BOT,
        out,
    )?;

    let cp = flatten_derived_grid(&scoreboard.simulate_derived_road(2));
    draw_derived_road(
        "[ Cockroach Pig ]",
        &cp,
        ROW_CP_TOP,
        ROW_CP_DATA,
        ROW_CP_BOT,
        out,
    )?;

    queue!(out, ResetColor)
}
