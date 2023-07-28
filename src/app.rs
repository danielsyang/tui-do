use ratatui::widgets::ListState;
use sqlx::{Pool, Sqlite};

use crate::database::{connection, Task, TaskCrud};

pub enum InputMode {
    Editing,
    Normal,
}

pub enum CursorPlacement {
    Description,
    DueDate,
}

pub struct MyApp {
    pub state: ListState,
    pub items: Vec<Task>,
    pub mode: InputMode,
    pub cursor_placement: CursorPlacement,
    pub input_description_value: String,
    pub input_due_date_value: String,
    pub db_connection: Pool<Sqlite>,
}

impl MyApp {
    pub async fn new() -> Self {
        MyApp {
            state: ListState::default(),
            items: vec![],
            mode: InputMode::Normal,
            input_description_value: String::new(),
            input_due_date_value: String::new(),
            db_connection: connection().await,
            cursor_placement: CursorPlacement::Description,
        }
    }

    pub async fn select_or_unselect(&mut self, finished: &bool) {
        let position = self.state.selected().unwrap_or(0);

        let task = self
            .items
            .get(position)
            .expect("Invalid item, array out of bound.");

        self.update_task(&task.id, finished).await;
        // Update UI
        self.get_tasks().await;
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
        match self.cursor_placement {
            CursorPlacement::Description => {
                self.input_description_value =
                    self.input_description_value.to_owned() + c.to_string().as_str()
            }
            CursorPlacement::DueDate => {
                self.input_due_date_value =
                    self.input_due_date_value.to_owned() + c.to_string().as_str()
            }
        }
    }

    pub fn backspace_input(&mut self) {
        match self.cursor_placement {
            CursorPlacement::Description => {
                let mut next_value = self.input_description_value.clone();
                next_value.pop();
                self.input_description_value = next_value;
            }
            CursorPlacement::DueDate => {
                let mut next_value = self.input_due_date_value.clone();
                next_value.pop();
                self.input_due_date_value = next_value;
            }
        }
    }

    pub async fn add_to_list(&mut self) {
        let curr = self.input_description_value.clone();
        if curr.is_empty() {
            return;
        }
        self.insert_task(&curr).await;
        self.get_tasks().await;
        self.input_description_value = String::new();
    }
}
