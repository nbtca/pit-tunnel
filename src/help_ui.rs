use crate::app::App;

use ratatui::{
    backend::Backend,
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, style::Style,
};

pub fn help_ui<B: Backend>(frame: &mut Frame<B>, _app: &mut App) {
    let line1 = Line::from(vec![
        Span::from("    1. Press"),
        Span::styled(" 'i' ", Style::default().fg(ratatui::style::Color::Yellow)),
        Span::from("into Input Mode\n"),
        ]);
    let line2 = Line::from(vec![
        Span::from("    2. Press"),
        Span::styled(" ESC ", Style::default().fg(ratatui::style::Color::Yellow)),
        Span::from("to exit Input Mode \n"),
        ]);
    let line3 = Line::from(vec![
        Span::from("    3. Press"),
        Span::styled(" 'm' ", Style::default().fg(ratatui::style::Color::Yellow)),
        Span::from("to focus on the message panel\n"),
        ]);
    let line4 = Line::from(vec![
        Span::from("    4. Press"),
        Span::styled(" 'u' ", Style::default().fg(ratatui::style::Color::Yellow)),
        Span::from("to focus on the user panel\n"),
        ]);
    let line5 = Line::from(vec![
        Span::from("    5. Press"),
        Span::styled(" 'h' ", Style::default().fg(ratatui::style::Color::Yellow)),
        Span::from("to show this help panel\n"),
        ]);
    let line6 = Line::from(vec![
        Span::from("    6. Press"),
        Span::styled(" 'q' ", Style::default().fg(ratatui::style::Color::Yellow)),
        Span::from("to quit\n"),
        ]);
    let help_text = vec![Line::from(""),line1, line2, line3, line4, line5, line6];
    let help_panel = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::from(" Help ")),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(help_panel, frame.size());
}
