use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    prelude::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::{CursorPlacement, InputMode, MyApp};

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut MyApp) {
    const INSTRUCTION_AREA: usize = 0;
    const TASK_AREA: usize = 1;
    const ERROR_AREA: usize = 2;

    let instructions = Paragraph::new(render_instructions(app))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default());

    let matrix_tasks = app
        .items
        .iter()
        .map(|item| {
            let checked_item = if item.finished {
                Cell::from("[x]")
            } else {
                Cell::from("[ ]")
            };
            let description = Cell::from(item.description.to_string());
            // For now, mocking due date with created_at value
            let due_date = Cell::from(item.created_at.date_naive().to_string());

            let row = vec![checked_item, description, due_date];
            Row::new(row).style(Style::default().fg(Color::Rgb(196, 196, 196)))
        })
        .collect::<Vec<_>>();

    let tasks_table = Table::new(matrix_tasks)
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
        )
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(75),
            Constraint::Percentage(15),
        ]);

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
            frame.render_stateful_widget(tasks_table, chunks[TASK_AREA], &mut app.state);
        }
        InputMode::Editing => {
            let constraints = match &app.input_error {
                Some(_) => vec![
                    Constraint::Percentage(7),
                    Constraint::Percentage(65),
                    Constraint::Max(1),
                    Constraint::Percentage(15),
                    Constraint::Percentage(7),
                ],
                None => vec![
                    Constraint::Percentage(7),
                    Constraint::Percentage(65),
                    Constraint::Percentage(15),
                    Constraint::Percentage(7),
                ],
            };

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(constraints.as_ref())
                .split(frame.size());

            let input_area = match &app.input_error {
                Some(_) => 3,
                None => 2,
            };

            let due_date_area = match &app.input_error {
                Some(_) => 4,
                None => 3,
            };

            match &app.input_error {
                Some(err) => {
                    let error_paragraph =
                        Paragraph::new(Span::styled(err, Style::default().fg(Color::Red)));

                    frame.render_widget(instructions, chunks[INSTRUCTION_AREA]);
                    frame.render_stateful_widget(tasks_table, chunks[TASK_AREA], &mut app.state);
                    frame.render_widget(error_paragraph, chunks[ERROR_AREA]);
                    frame.render_widget(input, chunks[input_area]);
                    frame.render_widget(due_date_input, chunks[due_date_area]);
                }
                None => {
                    frame.render_widget(instructions, chunks[INSTRUCTION_AREA]);
                    frame.render_stateful_widget(tasks_table, chunks[TASK_AREA], &mut app.state);
                    frame.render_widget(input, chunks[input_area]);
                    frame.render_widget(due_date_input, chunks[due_date_area]);
                }
            }

            match app.cursor_placement {
                CursorPlacement::Description => frame.set_cursor(
                    chunks[input_area].x + (app.input_description_value.len() as u16) + 1,
                    chunks[input_area].y + 1,
                ),
                CursorPlacement::DueDate => frame.set_cursor(
                    chunks[due_date_area].x + (app.input_due_date_value.len() as u16) + 1,
                    chunks[due_date_area].y + 1,
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
