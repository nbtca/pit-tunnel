use std::io;

use crossterm::event::{KeyCode, KeyEvent};
use chrono::prelude::*;

use crate::app::{App, Interface, Mode, Msg, NodeEvent};

pub fn main_event(app: &mut App, key: KeyEvent) -> io::Result<bool> {
    match app.current_mode {
        Mode::Main => match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('i') => {
                app.current_mode = Mode::Type;
            }
            KeyCode::Char('h') => {
                app.current_interface = Interface::Help;
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
            KeyCode::Char('h') => {
                app.current_interface = Interface::Help;
            }
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
            KeyCode::Char('h') => {
                app.current_interface = Interface::Help;
            }
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
        send_user: app.username.clone(),
        send_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        recv_user: String::from("a"),
        msg: app.input.clone(),
        is_public: true,
        is_info: false,
    };
    app.sender.send(NodeEvent::MsgSend(msg.clone())).expect("Error sending message");
    app.messages.push(msg);
    app.input.clear();
}

pub fn login_event(app: &mut App, key: KeyEvent) -> io::Result<i8> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            return Ok(2);
        }
        KeyCode::Char('h') => {
            app.current_interface = Interface::Help;
        }
        KeyCode::Char(c) => {
            app.username.push(c);
        }
        KeyCode::Backspace => {
            app.username.pop();
        }
        KeyCode::Enter => {
            app.current_interface = Interface::Main;
            return Ok(1);
        }
        _ => {}
    }
    return Ok(0);
}
pub fn help_event(app: &mut App, key: KeyEvent) -> io::Result<bool> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.current_interface = Interface::Main;
        }
        _ => {}
    }
    return Ok(false);
}
