mod app;
mod help_ui;
mod key_event;
mod login_ui;
mod main_ui;
use app::{App, Interface, Mode};
use help_ui::help_ui;
use key_event::{help_event, login_event, main_event};
use login_ui::login_ui;
use main_ui::main_ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::Backend,
    prelude::{CrosstermBackend, Terminal},
};
use std::io::{self, Result};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| match app.current_interface {
            Interface::Login => login_ui(f, app),
            Interface::Main => main_ui(f, app),
            Interface::Help => help_ui(f, app),
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            if let Ok(true) = match app.current_interface {
                Interface::Login => login_event,
                Interface::Main => main_event,
                Interface::Help => help_event,
            }(app, key)
            {
                return Ok(());
            }
        }
    }
}
