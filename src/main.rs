mod app;
mod ui;

use std::{io, time::Instant};

use app::MyApp;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::ui;

fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = MyApp::new();

    let r = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: MyApp) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|t| ui(t, &mut app)).unwrap();
    }
}
