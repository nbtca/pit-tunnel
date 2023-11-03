use std::sync::mpsc::Sender;

use crossterm::event::KeyEvent;
use serde::{Deserialize, Serialize};

pub enum Interface {
    Main,
    Login,
    Help,
}
pub enum Mode {
    Main,
    Type,
    User,
    Message,
}

pub enum NodeEvent {
    MsgRecv(Msg),
    Key(KeyEvent),
    MsgSend(Msg),
}

#[derive(Serialize, Deserialize)]
pub struct Msg {
    pub send_user: String,
    pub recv_user: String,
    pub send_time: String,
    pub msg: String,
    pub is_public: bool,
    pub is_info: bool,
}
impl Msg {
    pub fn clone(&self) -> Msg {
        Msg {
            send_user: self.send_user.clone(),
            recv_user: self.recv_user.clone(),
            send_time: self.send_time.clone(),
            msg: self.msg.clone(),
            is_public: self.is_public,
            is_info: self.is_info,
        }
    }
}

pub struct App {
    pub input: String,
    pub username: String,
    pub messages: Vec<Msg>,
    pub current_mode: Mode,
    pub current_interface: Interface,
    pub scroll: usize,
    pub sender: Sender<NodeEvent>,
}
impl App {
    pub fn new(sender: Sender<NodeEvent>) -> App {
        App {
            input: String::from("hello"),
            username: String::from("Solsist"),
            messages: Vec::new(),
            current_mode: Mode::Main,
            current_interface: Interface::Login,
            scroll: 0,
            sender,
        }
    }
}
