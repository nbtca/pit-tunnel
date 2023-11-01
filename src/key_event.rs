use std::io;

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Mode, Msg};

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
                send_message(app);
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

fn send_message(app: &mut App) {
    let msg: Msg = Msg {
        send_user: String::from("b"),
        send_time: String::from("2020-12-12"),
        recv_user: String::from("a"),
        msg: app.input.clone(),
        is_public: true,
        is_info: false,
    };
    app.messages.push(msg);
    app.input.clear();
}

pub fn login_event(_app: &mut App, _key: KeyEvent) -> io::Result<bool> {
    return Ok(false);
}
pub fn help_event(_app: &mut App, _key: KeyEvent) -> io::Result<bool> {
    return Ok(false);
}
