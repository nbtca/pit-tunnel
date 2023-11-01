use crate::app::App;

use ratatui::{
    backend::Backend,
    prelude::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub fn login_ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }

    let input_layout = centered_rect(centered_rect(frame.size(), 35, 35), 80, 30);

    let input = Paragraph::new(app.username.clone()).block(Block::default().borders(Borders::ALL));
    frame.set_cursor(
        input_layout.x + app.username.width() as u16 + 1,
        input_layout.y + 1,
    );

    frame.render_widget(
        Block::default()
            .borders(Borders::all())
            .title(" Enter Your Username "),
        centered_rect(frame.size(), 35, 35),
    );
    frame.render_widget(input, input_layout);
}
