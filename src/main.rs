mod app;
mod help_ui;
mod key_event;
mod login_ui;
mod main_ui;
use app::{App, Interface, Msg};
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
    sync::mpsc,
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
    let (sender, receiver) = mpsc::channel();

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
    let username = app.username.clone();
    thread::spawn(move || {
        let mut socket: tungstenite::WebSocket<
            tungstenite::stream::MaybeTlsStream<std::net::TcpStream>,
        > = connect(
            Url::parse(("ws://127.0.0.1:8080/ws?data={\"Name\":\"".to_owned() + &username + "\"}").as_str()).unwrap(),
        )
        .expect("Can't connect")
        .0;

        loop {
            let data = socket.read().expect("Error reading message");
            let msg: Msg = serde_json::from_str(&data.to_string()).unwrap();
            sender.send(msg).expect("Error sending message");
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

        if let Ok(msg) = receiver.try_recv() {
            app.messages.push(msg);
            continue;
        }

        if let Event::Key(key) = event::read()? {
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
    }
}
