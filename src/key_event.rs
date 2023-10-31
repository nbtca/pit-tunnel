use std::io;

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Mode};

pub fn main_event(app: &mut App, key: KeyEvent) -> io::Result<bool> {
    match app.current_mode {
        Mode::Main => match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('i') => {
                app.current_mode = Mode::Type;
            }
            KeyCode::Char('u') => {
                app.current_mode = Mode::User;
            }
            KeyCode::Char('m') => {
                app.current_mode = Mode::Message;
            }
            _ => {}
        },
        Mode::Type => match key.code {
            KeyCode::Esc => {
                app.current_mode = Mode::Main;
            }
            KeyCode::Char(c) => {
                app.input.push(c);
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Enter => {
                app.messages.push(app.input.clone());
                app.input.clear();
            }
            _ => {}
        },
        Mode::User => match key.code {
            KeyCode::Char('i') => {
                app.current_mode = Mode::Type;
            }
            KeyCode::Char('m') => {
                app.current_mode = Mode::Message;
            }
            KeyCode::Esc => {
                app.current_mode = Mode::Main;
            }
            _ => {}
        },
        Mode::Message => match key.code {
            KeyCode::Char('i') => {
                app.current_mode = Mode::Type;
            }
            KeyCode::Char('u') => {
                app.current_mode = Mode::User;
            }
            KeyCode::Esc => {
                app.current_mode = Mode::Main;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if app.scroll > 0 {
                    app.scroll -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.scroll < app.messages.len() - 1 {
                    app.scroll += 1;
                }
            }
            _ => {}
        },
    }
    return Ok(false);
}

pub fn login_event(app: &mut App, key: KeyEvent) -> io::Result<bool> {
    return Ok(false);
}
pub fn help_event(app: &mut App, key: KeyEvent) -> io::Result<bool> {
    return Ok(false);
}
