use std::collections::HashMap;
use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

mod input;
mod menu;

mod start_menu;

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut state = menu::MenuState::Start;
    let mut menus: HashMap<menu::MenuState, Box<dyn menu::Menu>> = HashMap::new();

    menus.insert(menu::MenuState::Start, Box::new(start_menu::StartMenu {}));

    let mut should_quit = false;
    while !should_quit {
        let current_menu = menus.get(&state).unwrap();
        terminal.draw(|frame| current_menu.ui(frame))?;
        should_quit = handle_events()?;
    }

    // cleanup
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
