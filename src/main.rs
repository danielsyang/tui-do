mod app;
mod database;
mod ui;

use std::{
    io,
    time::{Duration, Instant},
};

use app::{InputMode, MyApp};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use database::TaskCrud;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::ui;

#[tokio::main]
async fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = MyApp::new().await;
    app.get_tasks().await;
    let tick_rate = Duration::from_millis(250);

    let r = run_app(&mut terminal, app, tick_rate).await;

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();

    terminal.show_cursor().unwrap();

    if let Err(err) = r {
        println!("{}", err);
    }
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: MyApp,
    tick_rate: Duration,
) -> io::Result<()> {
    let last_tick = Instant::now();

    loop {
        terminal.draw(|t| ui(t, &mut app)).unwrap();

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout).unwrap() {
            match app.mode {
                InputMode::Normal => {
                    if let Event::Key(key) = event::read().unwrap() {
                        match key.code {
                            KeyCode::Char('w') => app.previous_item(),
                            KeyCode::Char('s') => app.next_item(),
                            KeyCode::Char('e') => app.select_or_unselect(&true).await,
                            KeyCode::Char('q') => app.select_or_unselect(&false).await,
                            KeyCode::Enter => app.mode = InputMode::Editing,
                            KeyCode::Esc => return Ok(()),
                            _ => {}
                        }
                    }
                }
                InputMode::Editing => {
                    if let Event::Key(key) = event::read().unwrap() {
                        match key.code {
                            KeyCode::Char(c) => app.set_input(&c),
                            KeyCode::Backspace => app.backspace_input(),
                            KeyCode::Enter => app.add_to_list().await,
                            KeyCode::Esc => app.mode = InputMode::Normal,
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
