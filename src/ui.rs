use crate::app::{App, Mode};
use crossterm::style::Stylize;
use ratatui::{
    backend::Backend,
    prelude::{Alignment, Constraint, CrosstermBackend, Direction, Layout, Terminal},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(frame.size());
    let layout2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Length(25), Constraint::Min(0)])
        .split(layout[0]);
    let input = Paragraph::new(app.input.clone())
        .style(match app.current_mode {
            Mode::Type => Style::default().fg(ratatui::style::Color::Green),
            _ => Style::default(),
        })
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    match app.current_mode {
        Mode::Type => {
            frame.set_cursor(layout[1].x + app.input.width() as u16 + 1, layout[1].y + 1);
        }
        _ => {}
    }
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" Users ")
            .style(match app.current_mode {
                Mode::User => Style::default().fg(ratatui::style::Color::Green),
                _ => Style::default(),
            }),
        layout2[0],
    );
    frame.render_widget(input, layout[1]);
    let mut messages = app
        .messages
        .iter()
        .map(|message| {
            let date = "2023-10-31 ".to_string();
            Line::from(vec![
                Span::styled(date, Style::default().fg(ratatui::style::Color::Blue)),
                Span::styled("user1 ",Style::default().fg(ratatui::style::Color::Green)),
                Span::from(message),
            ])
        })
        .collect::<Vec<_>>();
    messages.append(&mut vec![Line::from(vec![]); 2]);
    if (messages.len() as u16) > layout2[1].height {
        app.scroll = messages.len() - layout2[1].height as usize;
    }
    let messages_panel = Paragraph::new(messages)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::from(" Messages ")),
        )
        .style(match app.current_mode {
            Mode::Message => Style::default().fg(ratatui::style::Color::Green),
            _ => Style::default(),
        })
        .alignment(Alignment::Left)
        .scroll((app.scroll as u16, 0))
        .wrap(Wrap { trim: false });
    frame.render_widget(messages_panel, layout2[1]);
}
