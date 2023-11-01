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

#[derive(Serialize, Deserialize)]
pub struct Msg {
    pub send_user: String,
    pub recv_user: String,
    pub send_time: String,
    pub msg: String,
    pub is_public: bool,
    pub is_info: bool,
}

pub struct App {
    pub input: String,
    pub messages: Vec<Msg>,
    pub current_mode: Mode,
    pub current_interface: Interface,
    pub scroll: usize,
}
impl App {
    pub fn new() -> App {
        App {
            input: String::from("hello"),
            messages: Vec::new(),
            current_mode: Mode::Main,
            current_interface: Interface::Main,
            scroll: 0,
        }
    }
}
