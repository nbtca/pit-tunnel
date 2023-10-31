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
struct Msg {
    send_user: String,
    recv_user: String,
    send_time: String,
    msg: String,
    is_public: bool,
    is_info: bool,
}

pub struct App {
    pub input: String,
    pub messages: Vec<String>,
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
