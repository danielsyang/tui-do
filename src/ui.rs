use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::MyApp;

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut MyApp) {
    let items = app
        .items
        .iter()
        .map(|&i| {
            ListItem::new(vec![Spans::from(Span::styled(i, Style::default()))])
                .style(Style::default().fg(Color::Black).bg(Color::White))
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
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(widget_items, frame.size(), &mut app.state);
}
