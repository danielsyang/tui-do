use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    prelude::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{InputMode, MyApp};

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut MyApp) {
    let instructions = render_instructions(app);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(frame.size());

    const TASK_AREA: usize = 2;
    const INPUT_AREA: usize = 3;
    const INSTRUCTION_AREA: usize = 0;

    let items = app
        .items
        .iter()
        .map(|item| {
            let finished_icon = if item.finished { "[x]" } else { "[ ]" };
            let content = format!("{} {}", finished_icon, item.description);
            let line = Line::from(Span::styled(content, Style::default()));

            ListItem::new(vec![line]).style(Style::default().fg(Color::Rgb(196, 196, 196)))
        })
        .collect::<Vec<_>>();

    let widget_items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(80, 133, 57)))
                .title("Your To-Do List"),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(148, 130, 68))
                .add_modifier(Modifier::BOLD),
        );

    frame.render_stateful_widget(widget_items, chunks[TASK_AREA], &mut app.state);

    let input = Paragraph::new(app.input_value.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("To Do: "));

    frame.render_widget(input, chunks[INPUT_AREA]);
    frame.render_widget(instructions, chunks[INSTRUCTION_AREA]);

    match app.mode {
        InputMode::Normal => {}
        InputMode::Editing => frame.set_cursor(
            chunks[INPUT_AREA].x + (app.input_value.len() as u16) + 1,
            chunks[INPUT_AREA].y + 1,
        ),
    }
}

fn render_instructions<'a>(app: &mut MyApp) -> Paragraph<'a> {
    let instructions = match app.mode {
        InputMode::Normal => Line::from(vec![
            Span::raw("Press "),
            Span::styled("'Enter'", Style::default().fg(Color::Rgb(148, 130, 68))),
            Span::raw(" to enter insert mode, "),
            Span::raw("Press "),
            Span::styled("'Esc'", Style::default().fg(Color::Rgb(148, 130, 68))),
            Span::raw(" to quit, "),
            Span::raw("Press "),
            Span::styled("'w'", Style::default().fg(Color::Rgb(148, 130, 68))),
            Span::raw(" to move up and "),
            Span::styled("'s'", Style::default().fg(Color::Rgb(148, 130, 68))),
            Span::raw(" to move down, "),
            Span::raw("Press "),
            Span::styled("'e'", Style::default().fg(Color::Rgb(148, 130, 68))),
            Span::raw(" to check off a task, "),
            Span::raw("Press "),
            Span::styled("'q'", Style::default().fg(Color::Rgb(148, 130, 68))),
            Span::raw(" to uncheck a task."),
        ]),
        InputMode::Editing => Line::from(vec![
            Span::raw("Press "),
            Span::styled("'Enter'", Style::default().fg(Color::Rgb(148, 130, 68))),
            Span::raw(" to save your task, "),
            Span::raw("Press "),
            Span::styled("'Esc'", Style::default().fg(Color::Rgb(148, 130, 68))),
            Span::raw(" to return to normal mode."),
        ]),
    };

    Paragraph::new(instructions)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default())
}
