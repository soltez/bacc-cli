use crossterm::{
    cursor, queue,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::io::{self, Write};

use crate::model::scoreboard::ScoreboardCache;

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

pub fn draw_bead_plate(cache: &ScoreboardCache, out: &mut io::Stdout) -> io::Result<()> {
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

    let bead_plate = cache.bead_plate();
    for (i, bead) in bead_plate.iter().enumerate() {
        let outcome = bead & 0x03;
        let hand_val = (bead >> 4) & 0x0F;
        let col = i / BEAD_ROWS;
        let row = i % BEAD_ROWS;
        queue!(
            out,
            cursor::MoveTo(
                SCORE_COL_INNER_L + 1 + col as u16,
                ROW_BEAD_DATA + row as u16
            ),
            SetBackgroundColor(outcome_color(outcome)),
            SetForegroundColor(Color::White)
        )?;
        write!(out, "{hand_val}")?;
    }

    queue!(out, ResetColor)
}

fn simulate_big_road(columns: &[(u8, u8)]) -> [[Option<u8>; BIG_ROAD_ROWS]; BIG_ROAD_COLS] {
    let mut grid = [[None::<u8>; BIG_ROAD_ROWS]; BIG_ROAD_COLS];
    let mut next_col: usize = 0;

    for &(marker, count) in columns {
        // Rule 1: start in first column where row 0 is empty.
        let mut start = next_col;
        while start < BIG_ROAD_COLS && grid[start][0].is_some() {
            start += 1;
        }
        next_col = start.saturating_add(1);

        let mut col = start;
        let mut row: usize = 0;
        let mut going_down = true;
        let mut remaining = count as usize;

        while remaining > 0 {
            // Rule 4: scroll left when col overflows.
            while col >= BIG_ROAD_COLS {
                for c in 0..BIG_ROAD_COLS - 1 {
                    grid[c] = grid[c + 1];
                }
                grid[BIG_ROAD_COLS - 1] = [None; BIG_ROAD_ROWS];
                next_col = next_col.saturating_sub(1);
                col -= 1;
            }

            // Rule 3: same color directly below -- stop one row early, turn right.
            if going_down && row + 1 < BIG_ROAD_ROWS && grid[col][row + 1] == Some(marker) {
                going_down = false;
                col += 1;
                row = row.saturating_sub(1);
                continue;
            }

            // Place cell.
            grid[col][row] = Some(marker);
            remaining -= 1;

            // Rule 2: turn right when next row is occupied or bottom reached.
            if going_down {
                let nr = row + 1;
                if nr < BIG_ROAD_ROWS && grid[col][nr].is_none() {
                    row = nr;
                } else {
                    going_down = false;
                    col += 1;
                }
            } else {
                col += 1;
            }
        }
    }

    grid
}

pub fn draw_big_road(cache: &ScoreboardCache, out: &mut io::Stdout) -> io::Result<()> {
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

    let grid = simulate_big_road(cache.big_road());

    for (col, column) in grid.iter().enumerate() {
        for (row, cell) in column.iter().enumerate().take(BIG_ROAD_ROWS) {
            if let Some(marker) = cell {
                let bg = outcome_color(*marker);
                queue!(
                    out,
                    cursor::MoveTo(
                        SCORE_COL_INNER_L + 1 + col as u16,
                        ROW_BIG_ROAD_DATA + row as u16
                    ),
                    SetBackgroundColor(bg),
                    SetForegroundColor(Color::White)
                )?;
                write!(out, "\u{25CB}")?;
            }
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

pub fn draw_derived_roads(cache: &ScoreboardCache, out: &mut io::Stdout) -> io::Result<()> {
    draw_derived_road(
        "[ Big Eye Boy ]",
        cache.derived_road(0),
        ROW_BEB_TOP,
        ROW_BEB_DATA,
        ROW_BEB_BOT,
        out,
    )?;
    draw_derived_road(
        "[ Small Road ]",
        cache.derived_road(1),
        ROW_SR_TOP,
        ROW_SR_DATA,
        ROW_SR_BOT,
        out,
    )?;
    draw_derived_road(
        "[ Cockroach Pig ]",
        cache.derived_road(2),
        ROW_CP_TOP,
        ROW_CP_DATA,
        ROW_CP_BOT,
        out,
    )?;
    queue!(out, ResetColor)
}
