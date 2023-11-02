mod app;
mod help_ui;
mod key_event;
mod login_ui;
mod main_ui;
use app::{App, Interface, Msg, NodeEvent};
use help_ui::help_ui;
use key_event::{help_event, login_event, main_event};
use login_ui::login_ui;
use main_ui::main_ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::Backend,
    prelude::{CrosstermBackend, Terminal},
};
use std::{
    io::{self, Result},
    net::SocketAddr,
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};
use tungstenite::connect;
use url::Url;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (sender, _) = mpsc::channel();

    let mut app = App::new(sender);
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
    let (sender, receiver) = mpsc::channel();
    app.sender = sender.clone();

    loop {
        terminal.draw(|f| login_ui(f, app)).unwrap();
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match login_event(app, key).unwrap() {
                1 => break,
                2 => return Ok(()),
                _ => {}
            }
        }
    }

    let socket = Arc::new(Mutex::new(
        connect(
            Url::parse(
                ("ws://127.0.0.1:8080/ws?data={\"Name\":\"".to_owned() + &app.username + "\"}")
                    .as_str(),
            )
            .unwrap(),
        )
        .expect("Can't connect")
        .0,
    ));

    let socket_read = socket.clone();
    let socket_send = socket.clone();

    let sender_msg = sender.clone();
    let sender_key = sender.clone();

    // 接收消息并通过Channel发送
    thread::spawn(move || loop {
        let data = socket_read
            .clone()
            .lock()
            .unwrap()
            .read()
            .expect("Error reading message");
        let msg: Msg = serde_json::from_str(&data.to_string()).unwrap();
        sender_msg
            .send(NodeEvent::MsgRecv(msg))
            .expect("Error sending message");
    });

    // 接收按键事件并通过Channel发送
    thread::spawn(move || loop {
        if let Event::Key(key) = event::read().expect("Error reading key") {
            sender_key
                .send(NodeEvent::Key(key))
                .expect("Error sending message");
        }
    });

    loop {
        terminal
            .draw(|f| match app.current_interface {
                Interface::Login => login_ui(f, app),
                Interface::Main => main_ui(f, app),
                Interface::Help => help_ui(f, app),
            })
            .unwrap();

        if let Ok(event) = receiver.try_recv() {
            match event {
                NodeEvent::MsgRecv(msg) => {
                    app.messages.push(msg);
                }
                NodeEvent::Key(key) => {
                    if key.kind == event::KeyEventKind::Release {
                        continue;
                    }

                    if let Ok(true) = match app.current_interface {
                        Interface::Main => main_event,
                        Interface::Help => help_event,
                        _ => main_event,
                    }(app, key)
                    {
                        return Ok(());
                    }
                }
                NodeEvent::MsgSend(msg) => {
                    let mut socket = socket_send.lock().unwrap();
                    socket
                        .send(tungstenite::Message::Text(
                            serde_json::to_string(&msg).unwrap(),
                        ))
                        .expect("Error sending message");
                }
            }
        }
    }
}
