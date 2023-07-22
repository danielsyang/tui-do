use ratatui::widgets::ListState;

pub enum InputMode {
    Editing,
    Normal,
}

pub struct MyApp {
    pub state: ListState,
    pub items: Vec<String>,
    pub mode: InputMode,
    pub input_value: String,
}

impl MyApp {
    pub fn new() -> Self {
        MyApp {
            state: ListState::default(),
            items: vec![],
            mode: InputMode::Normal,
            input_value: String::new(),
        }
    }

    pub fn set_app_mode(&mut self, mode: InputMode) {
        self.mode = mode
    }

    pub fn unselect(&mut self) {
        self.state.select(None)
    }

    pub fn next_item(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i < self.items.len() - 1 {
                    i + 1
                } else {
                    self.items.len() - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i))
    }

    pub fn previous_item(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i))
    }

    pub fn set_input(&mut self, c: &char) {
        self.input_value = self.input_value.to_owned() + c.to_string().as_str();
    }

    pub fn backspace_input(&mut self) {
        let mut next_value = self.input_value.clone();
        next_value.pop();
        self.input_value = next_value;
    }

    pub fn add_to_list(&mut self) {
        let curr = self.input_value.clone();
        if curr.is_empty() {
            return;
        }

        self.items.push(curr);
        self.input_value = String::new();
    }
}
