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

use async_std::task;
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
use url::Url;
use websockets::WebSocket;

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (sender, _) = mpsc::channel();

    let mut app = App::new(sender);
    let res = run_app(&mut terminal, &mut app).await;

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

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
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
    let url = Url::parse(
        ("ws://127.0.0.1:8080/ws?data={\"Name\":\"".to_owned() + &app.username + "\"}").as_str(),
    )
    .clone()
    .unwrap();
    let mut ws = WebSocket::connect(url.as_str()).await.unwrap();
    let (mut read, mut write) = ws.split();
    let read = Arc::new(Mutex::new(read));
    // let socket = Arc::new(Mutex::new({
    //     // let (client, _) = tungstenite::client::client(url.clone(), tcp_stream).unwrap();
    //     // let (mut ws_stream, _) = connect(url.clone()).expect("Can't connect");
    //     // let client = match ws_stream.get_mut() {
    //     //     tt::Plain(tcp_stream) => {
    //     //         let (client, _) = tungstenite::client::client(url.clone(), tcp_stream).unwrap();
    //     //         client.read
    //     //         Option::Some(client.unwrap())
    //     //         //https://github.com/snapview/tungstenite-rs/issues/11
    //     //     }
    //     //     _ => Option::None,
    //     // };
    //     // let a = client.unwrap().0;
    //     // ws_stream
    // }));
    // let socket_read = socket.clone();
    // let socket_send = socket.clone();

    let sender_msg = sender.clone();
    let sender_key = sender.clone();

    // 接收消息并通过Channel发送
    //https://github.com/sdroege/async-tungstenite
    //https://github.com/snapview/tokio-tungstenite
    thread::spawn(move || async {
        loop {
            // let data = socket_read
            //     .clone()
            //     .lock()
            //     .unwrap()
            //     .read()
            //     .expect("Error reading message");
            let read = read.clone().lock().unwrap();
            let received = read.receive();
            let (data, _, _) = received.await.unwrap().as_text().unwrap();
            let msg: Msg = serde_json::from_str(&data).unwrap();
            sender_msg
                .send(NodeEvent::MsgRecv(msg))
                .expect("Error sending message");
        }
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
                    write
                        .send_text(serde_json::to_string(&msg).unwrap())
                        .await
                        .expect("Error sending message");
                    // sender.send(NodeEvent::MsgSend(msg)).unwrap();
                    // let mut socket = read
                    //     .socket
                    //     .send(tungstenite::Message::Text(
                    //         serde_json::to_string(&msg).unwrap(),
                    //     ))
                    //     .expect("Error sending message");
                }
            }
        }
    }
}
