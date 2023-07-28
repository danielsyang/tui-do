use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    prelude::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{CursorPlacement, InputMode, MyApp};

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut MyApp) {
    const INSTRUCTION_AREA: usize = 0;
    const TASK_AREA: usize = 1;
    const INPUT_AREA: usize = 2;
    const DUE_DATE_AREA: usize = 3;

    let instructions = Paragraph::new(render_instructions(app))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default());

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

    let input = Paragraph::new(app.input_description_value.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Description (*) "),
        );

    let due_date_input = Paragraph::new(app.input_due_date_value.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Due Date (DD-MM-YYYY or DD-MM-YYYY hh:mm) (default: EOD) "),
        );

    match app.mode {
        InputMode::Normal => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(7), Constraint::Percentage(100)])
                .split(frame.size());

            frame.render_widget(instructions, chunks[INSTRUCTION_AREA]);
            frame.render_stateful_widget(widget_items, chunks[TASK_AREA], &mut app.state);
        }
        InputMode::Editing => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(7),
                    Constraint::Percentage(70),
                    Constraint::Percentage(15),
                    Constraint::Percentage(7),
                ])
                .split(frame.size());

            frame.render_widget(instructions, chunks[INSTRUCTION_AREA]);
            frame.render_stateful_widget(widget_items, chunks[TASK_AREA], &mut app.state);
            frame.render_widget(input, chunks[INPUT_AREA]);
            frame.render_widget(due_date_input, chunks[DUE_DATE_AREA]);

            match app.cursor_placement {
                CursorPlacement::Description => frame.set_cursor(
                    chunks[INPUT_AREA].x + (app.input_description_value.len() as u16) + 1,
                    chunks[INPUT_AREA].y + 1,
                ),
                CursorPlacement::DueDate => frame.set_cursor(
                    chunks[DUE_DATE_AREA].x + (app.input_due_date_value.len() as u16) + 1,
                    chunks[DUE_DATE_AREA].y + 1,
                ),
            }
        }
    }
}

fn render_instructions<'a>(app: &mut MyApp) -> Line<'a> {
    match app.mode {
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
    }
}
