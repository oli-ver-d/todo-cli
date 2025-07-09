use chrono::Local;
use ratatui::widgets::ListState;

use crate::TodoItem;

pub struct App {
    pub todos: Vec<TodoItem>,
    pub list_state: ListState,
    pub mode: Mode,
    pub command: String,
}

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    New,
}

impl App {
    pub fn new(todos: Vec<TodoItem>) -> App {
        App {
            todos,
            list_state: ListState::default(),
            mode: Mode::Normal,
            command: String::new(),
        }
    }

    pub fn toggle_status(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.todos.remove(i);
        }
    }

    pub fn insert_new(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i < self.todos.len() - 1 {
                self.todos.insert(
                    i + 1,
                    TodoItem {
                        timestamp: Local::now(),
                        description: String::new(),
                    },
                );
            } else {
                self.todos.push(TodoItem {
                    timestamp: Local::now(),
                    description: String::new(),
                });
            }
        }
    }
}
