use crate::app::{App, Mode};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Style,
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    // Implement the UI
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    // Render the title at the top of the screen

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Todo-cli Interactive mode",
        Style::default().fg(ratatui::style::Color::White),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    // Render the status at bottom of the screen

    let status_text: String = match app.mode {
        Mode::Normal => "[NOR]".to_string(),
        Mode::Insert => "[INS]".to_string(),
        Mode::New => "[NEW]".to_string(),
        Mode::Command => format!(":{}", app.command),
    };

    let status_bar = Paragraph::new(Text::styled(
        status_text,
        Style::default()
            .bg(ratatui::style::Color::Black)
            .fg(ratatui::style::Color::White),
    ))
    .block(Block::default());

    frame.render_widget(status_bar, chunks[2]);

    let block = Block::new().borders(Borders::BOTTOM);

    let items: Vec<ListItem> = app
        .todos
        .iter()
        .enumerate()
        .map(|(i, todo_item)| ListItem::new(todo_item.description.clone()))
        .collect();

    let list = List::new(items).block(block).highlight_symbol("> ");

    frame.render_stateful_widget(list, chunks[1], &mut app.list_state);
}
