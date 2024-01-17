use ratatui::widgets::{ListState};

/// Application.
#[derive(Debug, Default)]
pub struct App {
  /// should the application exit?
  pub should_quit: bool,
  /// counter
  pub counter: i64,
  /// tasks
  pub tasks: Vec<Task>,
  /// current task
  pub current_task: ListState,
}

/// A list with a potentially-selected item
struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

/// Task.
#[derive(Debug, Default)]
pub struct Task {
  /// title
  pub title: String,
  /// has the task been completed?
  pub complete: bool,
  /// duration
  pub dur: u32,
}

impl App {
  /// Constructs a new instance of [`App`].
  pub fn new() -> Self {
    let mut app = Self::default();
    app.current_task.select(Some(0));
    app.tasks.push( Task {
      title: "brush teeth".to_string(),
      complete: false,
      dur: 180_000,
    });
    app.tasks.push( Task {
      title: "put on glasses".to_string(),
      complete: false,
      dur: 60_000,
    });
    app.tasks.push( Task {
      title: "turn on music".to_string(),
      complete: false,
      dur: 60_000,
    });
    app
  }

  /// Handles the tick event of the terminal.
  pub fn tick(&self) {}

  /// Set should_quit to true to quit the application.
  pub fn quit(&mut self) {
    self.should_quit = true;
  }

  pub fn increment_counter(&mut self) {
    if let Some(res) = self.counter.checked_add(1) {
      self.counter = res;
    }
  }

  pub fn decrement_counter(&mut self) {
    if let Some(res) = self.counter.checked_sub(1) {
      self.counter = res;
    }
  }
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
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

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_app_increment_counter() {
    let mut app = App::default();
    app.increment_counter();
    assert_eq!(app.counter, 1);
  }

  #[test]
  fn test_app_decrement_counter() {
    let mut app = App::default();
    app.decrement_counter();
    assert_eq!(app.counter, 0);
  }
}
