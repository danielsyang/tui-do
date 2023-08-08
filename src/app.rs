use ratatui::widgets::TableState;
use sqlx::{
    types::chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc},
    Pool, Sqlite,
};

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
    pub state: TableState,
    pub items: Vec<Task>,
    pub mode: InputMode,
    pub cursor_placement: CursorPlacement,
    pub input_description_value: String,
    pub input_due_date_value: String,
    pub input_error: Option<String>,
    pub db_connection: Pool<Sqlite>,
}

impl MyApp {
    pub async fn new() -> Self {
        MyApp {
            state: TableState::default(),
            items: vec![],
            mode: InputMode::Normal,
            cursor_placement: CursorPlacement::Description,
            input_description_value: String::new(),
            input_due_date_value: String::new(),
            input_error: None,
            db_connection: connection().await,
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
        let curr_description = self.input_description_value.clone();
        let curr_due_date = self.input_due_date_value.clone();
        if curr_description.is_empty() {
            return;
        }

        let parsed_due_date = if curr_due_date.is_empty() {
            Ok(Utc::now().naive_utc())
        } else {
            match NaiveDate::parse_from_str(curr_due_date.as_str(), "%d-%m-%Y") {
                Ok(date) => {
                    let time = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
                    Ok(NaiveDateTime::new(date, time))
                }
                Err(_) => NaiveDateTime::parse_from_str(curr_due_date.as_str(), "%d-%m-%Y %H:%M"),
            }
        };

        match parsed_due_date {
            Ok(due_date) => {
                self.insert_task(&curr_description, &due_date).await;
                self.get_tasks().await;
                self.input_description_value = String::new();
                self.input_due_date_value = String::new();
            }
            Err(_) => {
                self.input_error = Some("Due date is invalid".to_string());
                return;
            }
        }
    }
}
