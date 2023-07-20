use tui::widgets::ListState;

pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<(&'static str, usize)>,
}

impl StatefulList {}

pub struct MyApp {
    pub items: StatefulList,
    pub events: Vec<(&'static str, &'static str)>,
}

impl MyApp {
    pub fn new() -> Self {
        MyApp {
            items: StatefulList {
                state: ListState::default(),
                items: vec![],
            },
            events: vec![],
        }
    }
}
