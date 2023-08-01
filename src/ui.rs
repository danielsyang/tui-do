use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    prelude::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, List, ListItem, Padding, Paragraph, Row, Table, Wrap},
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

    // let items = app
    //     .items
    //     .iter()
    //     .map(|item| {
    //         let finished_icon = if item.finished { "[x]" } else { "[ ]" };
    //         let content = format!("{} {}", finished_icon, item.description);
    //         let line = Line::from(Span::styled(content, Style::default()));

    //         ListItem::new(vec![line]).style(Style::default().fg(Color::Rgb(196, 196, 196)))
    //     })
    //     .collect::<Vec<_>>();

    // let widget_items = List::new(items)
    //     .block(
    //         Block::default()
    //             .borders(Borders::ALL)
    //             .border_style(Style::default().fg(Color::Rgb(80, 133, 57)))
    //             .title("Your To-Do List"),
    //     )
    //     .highlight_style(
    //         Style::default()
    //             .fg(Color::Rgb(148, 130, 68))
    //             .add_modifier(Modifier::BOLD),
    //     );

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

    let t = vec![
        // Row can be created from simple strings.
        Row::new(vec!["Row11", "Row12", "Row13"]),
        // You can style the entire row.
        Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::default().fg(Color::Blue)),
        // If you need more control over the styling you may need to create Cells directly
        Row::new(vec![
            Cell::from("Row31"),
            Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
            Cell::from(Line::from(vec![
                Span::raw("Row"),
                Span::styled("33", Style::default().fg(Color::Green)),
            ])),
        ]),
        // If a Row need to display some content over multiple lines, you just have to change
        // its height.
        Row::new(vec![
            Cell::from("Row\n41"),
            Cell::from("Row\n42"),
            Cell::from("Row\n43"),
        ])
        .height(2),
    ];

    let t2 = vec![
        Row::new(vec![Cell::from("Thursday Jul 29")]),
        Row::new(vec!["[ ] Task One"]),
        Row::new(vec!["[ ] Task Two"]),
        Row::new(vec!["[ ] Task Three"]).bottom_margin(1),
        Row::new(vec![Cell::from("Thursday Jul 30")]),
        Row::new(vec!["[ ] Task Four"]),
        Row::new(vec!["[ ] Task Five"]).height(2),
        Row::new(vec!["[ ] Task Six"]).bottom_margin(1),
    ];

    let w = vec![
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(33),
    ];

    let w2 = vec![Constraint::Percentage(100)];

    let table = Table::new(t)
        // You can set the style of the entire Table.
        .style(Style::default().fg(Color::White))
        // It has an optional header, which is simply a Row always visible at the top.
        // As any other widget, a Table can be wrapped in a Block.
        .block(
            Block::default()
                .title("Your To-Do List")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(80, 133, 57)))
                .padding(Padding::new(1, 1, 1, 1)),
        )
        // Columns widths are constrained in the same way as Layout...
        .widths(w.as_ref())
        .highlight_symbol("> > ");
    // // If you wish to highlight a row in any specific way when it is selected...
    // .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    // // ...and potentially show a symbol in front of the selection.
    // .highlight_symbol(">>");

    // frame.render_widget(table, frame.size());
    frame.render_stateful_widget(table, frame.size(), &mut app.state);

    // match app.mode {
    //     InputMode::Normal => {
    //         let chunks = Layout::default()
    //             .direction(Direction::Vertical)
    //             .margin(1)
    //             .constraints([Constraint::Percentage(7), Constraint::Percentage(100)])
    //             .split(frame.size());

    //         frame.render_widget(instructions, chunks[INSTRUCTION_AREA]);
    //         frame.render_stateful_widget(widget_items, chunks[TASK_AREA], &mut app.state);
    //     }
    //     InputMode::Editing => {
    //         let constraints = match &app.input_error {
    //             Some(_) => vec![
    //                 Constraint::Percentage(7),
    //                 Constraint::Percentage(65),
    //                 Constraint::Max(1),
    //                 Constraint::Percentage(15),
    //                 Constraint::Percentage(7),
    //             ],
    //             None => vec![
    //                 Constraint::Percentage(7),
    //                 Constraint::Percentage(65),
    //                 Constraint::Percentage(15),
    //                 Constraint::Percentage(7),
    //             ],
    //         };

    //         let chunks = Layout::default()
    //             .direction(Direction::Vertical)
    //             .margin(1)
    //             .constraints(constraints.as_ref())
    //             .split(frame.size());

    //         let input_area = match &app.input_error {
    //             Some(_) => 3,
    //             None => 2,
    //         };

    //         let due_date_area = match &app.input_error {
    //             Some(_) => 4,
    //             None => 3,
    //         };

    //         match &app.input_error {
    //             Some(err) => {
    //                 let error_paragraph =
    //                     Paragraph::new(Span::styled(err, Style::default().fg(Color::Red)));

    //                 frame.render_widget(instructions, chunks[INSTRUCTION_AREA]);
    //                 frame.render_stateful_widget(widget_items, chunks[TASK_AREA], &mut app.state);
    //                 frame.render_widget(error_paragraph, chunks[ERROR_AREA]);
    //                 frame.render_widget(input, chunks[input_area]);
    //                 frame.render_widget(due_date_input, chunks[due_date_area]);
    //             }
    //             None => {
    //                 frame.render_widget(instructions, chunks[INSTRUCTION_AREA]);
    //                 frame.render_stateful_widget(widget_items, chunks[TASK_AREA], &mut app.state);
    //                 frame.render_widget(input, chunks[input_area]);
    //                 frame.render_widget(due_date_input, chunks[due_date_area]);
    //             }
    //         }

    //         match app.cursor_placement {
    //             CursorPlacement::Description => frame.set_cursor(
    //                 chunks[input_area].x + (app.input_description_value.len() as u16) + 1,
    //                 chunks[input_area].y + 1,
    //             ),
    //             CursorPlacement::DueDate => frame.set_cursor(
    //                 chunks[due_date_area].x + (app.input_due_date_value.len() as u16) + 1,
    //                 chunks[due_date_area].y + 1,
    //             ),
    //         }
    //     }
    // }
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
