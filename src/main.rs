mod controller;
mod model;
mod view;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, terminal,
};
use std::io;
use std::time::Duration;

use controller::bet::{handle_bet_backspace, handle_bet_enter, handle_bet_escape, handle_bet_key};
use controller::deal::{advance_deal, auto_advance_delay_ms, handle_enter, should_auto_advance};
use controller::settings::{toggle_peel_enabled, toggle_show_hands};
use model::game::Game;
use view::render::render;

fn run() -> io::Result<()> {
    let mut out = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(out, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut game = Game::new();

    loop {
        render(&game, &mut out)?;

        if should_auto_advance(&game)
            && !event::poll(Duration::from_millis(auto_advance_delay_ms(&game)))?
        {
            advance_deal(&mut game);
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            break;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => break,
            KeyCode::Enter => {
                if game.bet().input().is_some() {
                    handle_bet_enter(&mut game);
                } else {
                    handle_enter(&mut game);
                }
            }
            KeyCode::Backspace => handle_bet_backspace(&mut game),
            KeyCode::Esc => handle_bet_escape(&mut game),
            KeyCode::Char('h') | KeyCode::Char('H') => {
                toggle_show_hands(&mut game);
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                toggle_peel_enabled(&mut game);
            }
            KeyCode::Char(ch) => {
                handle_bet_key(&mut game, ch);
            }
            _ => {}
        }
    }

    execute!(out, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
