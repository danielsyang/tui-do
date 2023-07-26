use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{InputMode, MyApp};

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut MyApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(frame.size());

    let items = app
        .items
        .iter()
        .map(|item| {
            let finished_icon = if item.finished { "[x]" } else { "[ ]" };
            let content = format!("{} {}", finished_icon, item.description);
            let line = Line::from(Span::styled(content, Style::default()));

            ListItem::new(vec![line]).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect::<Vec<_>>();

    let widget_items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Your To-Do List"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_stateful_widget(widget_items, chunks[0], &mut app.state);

    let input = Paragraph::new(app.input_value.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("To Do: "));

    frame.render_widget(input, chunks[1]);

    match app.mode {
        InputMode::Normal => {}
        InputMode::Editing => frame.set_cursor(
            chunks[1].x + (app.input_value.len() as u16) + 1,
            chunks[1].y + 1,
        ),
    }
}
