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

use controller::deal::{advance_deal, deal_hand};
use model::game::Game;
use view::render::render;

fn run() -> io::Result<()> {
    let mut out = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(out, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut game = Game::new();

    loop {
        render(&game, &mut out)?;

        if game.round().should_auto_advance()
            && !event::poll(Duration::from_millis(game.display().deal_speed_ms()))?
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
                if game.round().phase() != 0 && game.round().complete() {
                    game.round_mut().end_round();
                } else if game.round().phase() == 0 {
                    deal_hand(&mut game);
                } else {
                    advance_deal(&mut game);
                }
            }
            KeyCode::Char('h') | KeyCode::Char('H') => {
                game.display_mut().toggle_show_hands();
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                game.display_mut().toggle_peel_enabled();
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
