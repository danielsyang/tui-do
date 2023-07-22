use tui::widgets::ListState;

pub struct MyApp {
    pub state: ListState,
    pub items: Vec<&'static str>,
}

impl MyApp {
    pub fn new() -> Self {
        MyApp {
            state: ListState::default(),
            items: vec![
                "Item0 Lorem ipsum blablalbla",
                "Item1 Lorem ipsum blablalbla",
                "Item2 Lorem ipsum blablalbla",
                "Item3 Lorem ipsum blablalbla",
                "Item4 Lorem ipsum blablalbla",
                "Item5 Lorem ipsum blablalbla",
                "Item6 Lorem ipsum blablalbla",
                "Item7 Lorem ipsum blablalbla",
                "Item8 Lorem ipsum blablalbla",
                "Item9 Lorem ipsum blablalbla",
                "Item10 Lorem ipsum blablalbla",
                "Item11 Lorem ipsum blablalbla",
                "Item12 Lorem ipsum blablalbla",
                "Item13 Lorem ipsum blablalbla",
                "Item14 Lorem ipsum blablalbla",
                "Item15 Lorem ipsum blablalbla",
                "Item16 Lorem ipsum blablalbla",
                "Item17 Lorem ipsum blablalbla",
                "Item18 Lorem ipsum blablalbla",
                "Item19 Lorem ipsum blablalbla",
                "Item20 Lorem ipsum blablalbla",
                "Item21 Lorem ipsum blablalbla",
                "Item22 Lorem ipsum blablalbla",
                "Item23 Lorem ipsum blablalbla",
                "Item24 Lorem ipsum blablalbla",
            ],
        }
    }

    pub fn unselect(&mut self) {
        self.state.select(None)
    }

    pub fn next_item(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == self.items.len() - 1 {
                    0
                } else {
                    i + 1
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
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i))
    }
}
