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
use std::io::Error;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::Backend,
    prelude::{CrosstermBackend, Terminal},
};
use std::io::{self, Result};
use url::Url;
use websockets::WebSocket;

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> () {
    let (sc, rc) = mpsc::channel();
    thread::spawn(move || async {
        let username = app.username.clone();
        let mut ws = WebSocket::connect(
            ("ws://127.0.0.1:8080/ws?data={\"Name\":\"".to_owned() + &username + "\"}").as_str(),
        )
        .await
        .unwrap();
        let (mut r, mut w) = ws.split();
        // let send_str = "你想提交的内容";
        // w.send_text(send_str.to_string()).await.unwrap();

        // 定期推送ping，如果想改这里自己建立信息通讯mpsc::channel(0) 发送不同的数据;
        thread::spawn(move || {
            loop {
                // 这里因为是async自己包裹一下不然跑步起来
                tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        // 睡眠定期推送数据
                        // thread::sleep(Duration::new(20, 0));
                        // w.send_text("ping".parse().unwrap()).await.unwrap();
                    });
            }
        });
        // 循环打印数据
        loop {
            let s = r.receive().await.unwrap();
            let (ss, b, snake_case) = s.as_text().unwrap();
        }
    });
    // let (sender, receiver) = mpsc::channel();

    // loop {
    //     terminal.draw(|f| login_ui(f, app)).unwrap();
    //     if let Event::Key(key) = event::read()? {
    //         if key.kind == event::KeyEventKind::Release {
    //             continue;
    //         }
    //         match login_event(app, key).unwrap() {
    //             1 => break,
    //             2 => return Ok(()),
    //             _ => {}
    //         }
    //     }
    // }
    // let username = app.username.clone();
    // thread::spawn(move || {
    //     let mut socket: tungstenite::WebSocket<
    //         tungstenite::stream::MaybeTlsStream<std::net::TcpStream>,
    //     > = connect(
    //         Url::parse(("ws://127.0.0.1:8080/ws?data={\"Name\":\"".to_owned() + &username + "\"}").as_str()).unwrap(),
    //     )
    //     .expect("Can't connect")
    //     .0;

    //     loop {
    //         let data = socket.read().expect("Error reading message");
    //         let msg: Msg = serde_json::from_str(&data.to_string()).unwrap();
    //         sender.send(msg).expect("Error sending message");
    //     }
    // });

    // loop {
    //     terminal
    //         .draw(|f| match app.current_interface {
    //             Interface::Login => login_ui(f, app),
    //             Interface::Main => main_ui(f, app),
    //             Interface::Help => help_ui(f, app),
    //         })
    //         .unwrap();

    //     if let Ok(msg) = receiver.try_recv() {
    //         app.messages.push(msg);
    //         continue;
    //     }

    //     if let Event::Key(key) = event::read()? {
    //         if key.kind == event::KeyEventKind::Release {
    //             continue;
    //         }
    //         if let Ok(true) = match app.current_interface {
    //             Interface::Main => main_event,
    //             Interface::Help => help_event,
    //             _ => main_event,
    //         }(app, key)
    //         {
    //             return Ok(());
    //         }
    //     }
    // }
}
