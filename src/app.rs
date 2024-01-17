use ratatui::widgets::{TableState};

use chrono::{Duration};

/// Application.
#[derive(Debug, Default)]
pub struct App {
  /// should the application exit?
  pub should_quit: bool,
  /// counter
  pub counter: i64,
  /// tasks
  pub tasks: StatefulList<Task>,
}

/// A list with a potentially-selected item
#[derive(Debug, Default)]
pub struct StatefulList<T> {
    pub state: TableState,
    pub items: Vec<T>,
}

/// Task.
#[derive(Debug)]
pub struct Task {
  /// title
  pub title: String,
  /// has the task been completed?
  pub complete: bool,
  /// duration
  pub dur: Duration,
}

impl App {
  /// Constructs a new instance of [`App`].
  pub fn new() -> Self {
    let mut app = Self::default();
    app.tasks.state.select(Some(0));
    app.tasks.items.push( Task {
      title: "brush teeth".to_string(),
      complete: false,
      dur: Duration::seconds(180),
    });
    app.tasks.items.push( Task {
      title: "put on glasses".to_string(),
      complete: false,
      dur: Duration::seconds(60),
    });
    app.tasks.items.push( Task {
      title: "turn on music".to_string(),
      complete: false,
      dur: Duration::seconds(60),
    });
    app
  }

  /// Handles the tick event of the terminal.
  pub fn tick(&self) {}

  /// Set should_quit to true to quit the application.
  pub fn quit(&mut self) {
    self.should_quit = true;
  }

  pub fn next_task(&mut self) {
    self.tasks.next();
  }

  pub fn prev_task(&mut self) {
    self.tasks.previous();
  }
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: TableState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
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
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            title: "Undefined".to_string(),
            complete: false,
            dur: Duration::minutes(1),
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;
}
