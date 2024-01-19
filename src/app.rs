use ratatui::widgets::TableState;

use std::time::{Instant, Duration};

/// Application.
#[derive(Debug)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// counter
    pub counter: i64,
    /// tasks
    pub tasks: StatefulList,
    /// routine start time
    pub start_time: Instant,
}

/// A list with a potentially-selected item
#[derive(Debug, Default)]
pub struct StatefulList {
    pub state: TableState,
    pub items: Vec<Task>,
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
    /// time started
    pub start_time: Instant,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut app = Self {
            should_quit: false,
            counter: 0,
            tasks: StatefulList::default(),
            start_time: Instant::now(),
        };

        app.tasks.state.select(Some(0));
        app.tasks.items.push(Task {
            title: "brush teeth".to_string(),
            complete: false,
            dur: Duration::from_secs(180),
            start_time: Instant::now(),
        });
        app.tasks.items.push(Task {
            title: "put on glasses".to_string(),
            complete: false,
            dur: Duration::from_secs(60),
            start_time: Instant::now(),
        });
        app.tasks.items.push(Task {
            title: "turn on music".to_string(),
            complete: false,
            dur: Duration::from_secs(60),
            start_time: Instant::now(),
        });
        app
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn get_time_elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn attempt_toggle(&mut self) {
        self.tasks.toggle_current();
    }

    pub fn next_task(&mut self) {
        self.tasks.next();
    }

    pub fn prev_task(&mut self) {
        self.tasks.previous();
    }
}

impl StatefulList {
    fn with_items(items: Vec<Task>) -> StatefulList {
        StatefulList {
            state: TableState::default(),
            items,
        }
    }

    fn get_current(&mut self) -> Option<&mut Task> {
        match self.state.selected() {
            Some(i) => self.items.get_mut(i),
            None => None,
        }
    }

    fn toggle_current(&mut self) {
        if let Some(i) = self.get_current() {
            i.complete = !i.complete;
        };
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
            dur: Duration::from_secs(60),
            start_time: Instant::now(),
        }
    }
}

impl Task {
    pub fn get_remaining_time(&self) -> Duration {
        self.dur.saturating_sub(Instant::now() - self.start_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
