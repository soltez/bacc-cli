use crossterm::{
    cursor, queue,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::io::{self, Write};

const COL_OPT_L: u16 = 63;
const COL_OPT_R: u16 = 79;
const OPT_INNER_W: usize = (COL_OPT_R - COL_OPT_L - 1) as usize; // = 15
const ROW_OPT_TOP: u16 = 9;
const ROW_OPT_BOT: u16 = 14;

pub fn draw_options_panel(
    show_hands: bool,
    peel_enabled: bool,
    auto_run_digits: Option<&str>,
    out: &mut io::Stdout,
) -> io::Result<()> {
    draw_top_border(out)?;
    // prefix + key + rest + yn = 13 chars (1-char padding each side inside 15-char inner)
    draw_option_row(ROW_OPT_TOP + 1, "", 'H', "ands On    ", show_hands, out)?;
    draw_option_row(ROW_OPT_TOP + 2, "p", 'E', "el Card   ", peel_enabled, out)?;
    draw_shortcut_row(ROW_OPT_TOP + 3, out)?;
    if let Some(digits) = auto_run_digits {
        draw_auto_run_input_row(ROW_OPT_TOP + 4, digits, out)?;
    } else {
        draw_enter_row(ROW_OPT_TOP + 4, out)?;
    }
    draw_bottom_border(out)?;
    queue!(out, ResetColor)
}

fn draw_top_border(out: &mut io::Stdout) -> io::Result<()> {
    let header = "[ Options ]"; // 11 chars
    let hfill = OPT_INNER_W.saturating_sub(header.len());
    let hl = hfill / 2;
    let hr = hfill - hl;

    queue!(
        out,
        cursor::MoveTo(COL_OPT_L, ROW_OPT_TOP),
        SetBackgroundColor(Color::DarkCyan),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2554}{}", "\u{2550}".repeat(hl))?;
    write!(out, "[ ")?;
    queue!(out, SetForegroundColor(Color::Black))?;
    write!(out, "Options")?;
    queue!(out, SetForegroundColor(Color::DarkGrey))?;
    write!(out, " ]{}\u{2557}", "\u{2550}".repeat(hr))
}

fn draw_option_row(
    row: u16,
    prefix: &str,
    key: char,
    rest: &str,
    value: bool,
    out: &mut io::Stdout,
) -> io::Result<()> {
    let yn = if value { 'Y' } else { 'N' };
    queue!(
        out,
        cursor::MoveTo(COL_OPT_L, row),
        SetBackgroundColor(Color::DarkCyan),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2551} ")?;
    queue!(out, SetForegroundColor(Color::Yellow))?;
    write!(out, "{prefix}")?;
    queue!(out, SetForegroundColor(Color::Cyan))?;
    write!(out, "{key}")?;
    queue!(out, SetForegroundColor(Color::Yellow))?;
    write!(out, "{rest}{yn}")?;
    queue!(out, SetForegroundColor(Color::DarkGrey))?;
    write!(out, " \u{2551}")
}

fn draw_shortcut_row(row: u16, out: &mut io::Stdout) -> io::Result<()> {
    queue!(
        out,
        cursor::MoveTo(COL_OPT_L, row),
        SetBackgroundColor(Color::DarkCyan),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2551} ")?;
    queue!(out, SetForegroundColor(Color::Cyan))?;
    write!(out, "A")?;
    queue!(out, SetForegroundColor(Color::Yellow))?;
    write!(out, "uto Run ")?;
    queue!(out, SetForegroundColor(Color::Cyan))?;
    write!(out, "Q")?;
    queue!(out, SetForegroundColor(Color::Yellow))?;
    write!(out, "uit")?;
    queue!(out, SetForegroundColor(Color::DarkGrey))?;
    write!(out, " \u{2551}")
}

fn draw_enter_row(row: u16, out: &mut io::Stdout) -> io::Result<()> {
    queue!(
        out,
        cursor::MoveTo(COL_OPT_L, row),
        SetBackgroundColor(Color::DarkCyan),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2551} ")?;
    queue!(out, SetForegroundColor(Color::Cyan))?;
    write!(out, "[Enter]")?;
    queue!(out, SetForegroundColor(Color::Yellow))?;
    write!(out, " Deals")?;
    queue!(out, SetForegroundColor(Color::DarkGrey))?;
    write!(out, " \u{2551}")
}

fn draw_auto_run_input_row(row: u16, digits: &str, out: &mut io::Stdout) -> io::Result<()> {
    queue!(
        out,
        cursor::MoveTo(COL_OPT_L, row),
        SetBackgroundColor(Color::DarkRed),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{2551} ")?;
    queue!(out, SetForegroundColor(Color::Yellow))?;
    write!(out, "Hands:{:>7}", digits)?;
    queue!(out, SetForegroundColor(Color::DarkGrey))?;
    write!(out, " \u{2551}")
}

fn draw_bottom_border(out: &mut io::Stdout) -> io::Result<()> {
    queue!(
        out,
        cursor::MoveTo(COL_OPT_L, ROW_OPT_BOT),
        SetBackgroundColor(Color::DarkCyan),
        SetForegroundColor(Color::DarkGrey)
    )?;
    write!(out, "\u{255A}{}\u{255D}", "\u{2550}".repeat(OPT_INNER_W))
}
