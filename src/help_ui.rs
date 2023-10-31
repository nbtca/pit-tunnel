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

pub fn help_ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {

}
