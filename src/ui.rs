use tui::{
    backend::Backend,
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::MyApp;

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut MyApp) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    let items = app
        .items
        .items
        .iter()
        .map(|i| {
            let mut lines = vec![Spans::from(i.0)];

            for _ in 0..i.1 {
                lines.push(Spans::from(Span::styled(
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                    Style::default().add_modifier(Modifier::ITALIC),
                )))
            }

            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect::<Vec<_>>();

    let widget_items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(widget_items, chunks[0], &mut app.items.state);

    let events = app
        .events
        .iter()
        .map(|&(event, level)| {
            let s = match level {
                "CRITICAL" => Style::default().fg(Color::Red),
                "ERROR" => Style::default().fg(Color::Magenta),
                "WARNING" => Style::default().fg(Color::Yellow),
                "INFO" => Style::default().fg(Color::Green),
                _ => Style::default(),
            };

            let header = Spans::from(vec![
                Span::styled(format!("{:<9}", level), s),
                Span::raw(" "),
                Span::styled(
                    "2020-01-01 10:00:00",
                    Style::default().add_modifier(Modifier::ITALIC),
                ),
            ]);

            let logs = Spans::from(vec![Span::raw(event)]);

            ListItem::new(vec![
                Spans::from("-".repeat(chunks[1].width as usize)),
                header,
                Spans::from(""),
                logs,
            ])
        })
        .collect::<Vec<_>>();

    let events_list = List::new(events)
        .block(Block::default().borders(Borders::ALL).title("Events"))
        .start_corner(Corner::BottomLeft);

    frame.render_widget(events_list, chunks[1]);
}
