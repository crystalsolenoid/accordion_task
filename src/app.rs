use ratatui::widgets::TableState;

mod parse_routine;
pub mod static_task;

use static_task::{StaticTaskList, StaticTask};

use chrono::{DateTime, Local, Timelike};
use std::time::{Duration, Instant};

/// Application.
#[derive(Debug)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    pub debug: bool,
    /// counter
    pub counter: i64,
    /// task display widget
    pub task_widget_state: TableState,
    /// task internal list
    pub tasks: StaticTaskList,
    /// routine timer
    pub routine_timer: Timer,
    pub last_tick: Instant,
}

/// A list with a potentially-selected item
#[derive(Debug, Default)]
pub struct StatefulList<'a> {
    pub state: TableState,
    pub items: Vec<&'a StaticTask>,
}

/// Task.
#[derive(Debug)]
pub struct Task {
    /// title
    pub title: String,
    /// has the task been completed?
    pub complete: bool,
    /// task-specific timer
    pub timer: Timer,
}

/// Timer.
#[derive(Debug)]
pub struct Timer {
    /// duration
    pub duration: Duration,
    pub original_duration: Duration,
    /// time started
    pub start_instant: Instant,
    pub start_time: DateTime<Local>,
    /// time elapsed while unpaused
    pub elapsed: Duration,
    /// is the timer running?
    pub active: bool,
}

pub enum SignedDuration {
    SURPLUS(Duration),
    DEFICIT(Duration),
    ZERO,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> App {
        let tasks = StaticTaskList::with_tasks(parse_routine::read_csv().expect("Failed to load routine file"));
        let mut app = Self {
            should_quit: false,
            debug: false,
            counter: 0,
            tasks,
            task_widget_state: TableState::default(),
            routine_timer: Timer::default(),
            last_tick: Instant::now(),
        };
        app.routine_timer = Timer::from_duration(app.get_total_remaining());

        app.task_widget_state.select(app.tasks.active);
        app.start_routine();

        app
    }

    pub fn get_time_balance(&self) -> SignedDuration {
        // Report the amount ahead of or behind schedule
        // based on the routine timer and task timers.
        // To avoid timing noise, consider all durations
        // less than 1 second long to be SignedDuration::ZERO.
        // Because of this quirk, be careful using this
        // function for cumulative operations.
        let remaining = self.routine_timer.get_remaining();
        let to_do = self.get_total_remaining();
        if to_do > remaining {
            let difference = to_do - remaining;
            if difference.as_secs() > 0 {
                SignedDuration::DEFICIT(difference)
            } else {
                SignedDuration::ZERO
            }
        } else if to_do < remaining {
            let difference = remaining - to_do;
            if difference.as_secs() > 0 {
                SignedDuration::SURPLUS(difference)
            } else {
                SignedDuration::ZERO
            }
        } else {
            SignedDuration::ZERO
        }
    }

    pub fn get_total_remaining(&self) -> Duration {
        self.tasks
            .tasks
            .iter()
            .filter(|task| !task.complete)
            .map(|task| task.remaining())
            .sum()
    }

    pub fn get_total_duration(&self) -> Duration {
        self.tasks
            .tasks
            .iter()
            .filter(|task| !task.complete)
            .map(|task| task.duration())
            .sum()
    }

    pub fn get_unused_time(&self) -> Duration {
        self.tasks
            .tasks
            .iter()
            .filter(|task| task.complete)
            .map(|task| task.remaining())
            .sum()
    }

    pub fn get_start_time(&self) -> DateTime<Local> {
        self.routine_timer.start_time
    }

    pub fn get_projected_end_time(&self) -> DateTime<Local> {
        Local::now() + self.get_total_remaining()
    }

    pub fn start_routine(&mut self) {
        self.routine_timer.start();
        /*
        if let Some(mut i) = self.tasks.get_current() {
            i.timer.start();
        }
        */
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        let this_tick = Instant::now();
        let delta = this_tick - self.last_tick;
        self.last_tick = this_tick;

        self.tasks.elapse(delta);
        /*
        // shrink or stretch to time available
        let todo = self.get_total_duration().as_secs() as f64;
        let remaining = self.routine_timer.get_remaining().as_secs() as f64;
        match self.get_time_balance() {
            SignedDuration::ZERO => (),
            _ => {
                let ratio = match remaining / todo {
                    ..=0.0 => 0.0,
                    1.0.. => 1.0,
                    r => r,
                };
                for mut task in &mut self.tasks.items {
                    if !task.complete {
                        task.timer.adjust_duration(ratio);
                    }
                }
            }
        }
        */
        // TODO
    }

    pub fn get_time_elapsed(&self) -> Duration {
        self.routine_timer.start_instant.elapsed()
    }

    pub fn get_percentage_elapsed(&self) -> f64 {
        self.routine_timer.get_percentage()
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }

    pub fn attempt_toggle(&mut self) {
        self.tasks.toggle_current();
        self.task_widget_state.select(self.tasks.active);
    }

    pub fn next_task(&mut self) {
        // TODO update version of ratatui
//        self.task_widget_state.select_next();
        if let Some(i) = self.task_widget_state.selected() {
            let j = if i >= self.tasks.tasks.len() - 1 {
                0
            } else {
                i + 1
            };
            self.task_widget_state.select(Some(j));
            self.tasks.active = self.task_widget_state.selected();
        }
    }

    pub fn prev_task(&mut self) {
        // TODO update version of ratatui
//        self.task_widget_state.select_previous();
        if let Some(i) = self.task_widget_state.selected() {
            let j = if i == 0 {
                self.tasks.tasks.len() - 1
            } else {
                i - 1
            };
            self.task_widget_state.select(Some(j));
            self.tasks.active = self.task_widget_state.selected();
        }
    }
}

impl StatefulList<'_> {
    fn with_items<'a>(items: Vec<&'a StaticTask>) -> StatefulList<'a> {
        StatefulList {
            state: TableState::default(),
            items,
        }
    }

    /*
    fn get_current(&mut self) -> Option<&mut StaticTask> {
        match self.state.selected() {
            Some(i) => self.items.get_mut(i),
            None => None,
        }
    }

    fn toggle_current(&mut self) {
        if let Some(i) = self.get_current() {
            match i.complete {
                true => i.unset_complete(),
                false => {
                    i.set_complete();
                    self.next_no_wrap();
                }
            };
        };
    }

    fn next_no_wrap(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i < self.items.len() - 1 {
                    self.next()
                }
            }
            None => (),
        };
    }

    fn next(&mut self) {
        if let Some(i) = self.get_current() {
            i.deselect();
        }
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
        if let Some(i) = self.get_current() {
            i.select();
        }
    }

    fn previous(&mut self) {
        if let Some(i) = self.get_current() {
            i.deselect();
        }
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
        if let Some(i) = self.get_current() {
            i.select();
        }
    }

    fn unselect(&mut self) {
        if let Some(i) = self.get_current() {
            i.deselect();
        }
        self.state.select(None);
    }
    */
}

impl Default for Task {
    fn default() -> Self {
        Self {
            title: "Undefined".to_string(),
            complete: false,
            timer: Timer::from_secs(60),
        }
    }
}

impl Task {
    pub fn from_secs(seconds: u64) -> Self {
        Self {
            timer: Timer::from_secs(seconds),
            ..Self::default()
        }
    }

    pub fn get_remaining_time(&self) -> Duration {
        self.timer.get_remaining()
    }

    fn set_complete(&mut self) {
        self.complete = true;
        self.timer.pause();
    }

    fn unset_complete(&mut self) {
        self.complete = false;
        self.timer.start();
    }

    fn select(&mut self) {
        if !self.complete {
            self.timer.start();
        }
    }

    fn deselect(&mut self) {
        self.timer.pause();
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(5 * 60),
            original_duration: Duration::from_secs(5 * 60),
            start_instant: Instant::now(),
            start_time: Local::now(),
            elapsed: Duration::ZERO,
            active: false,
        }
    }
}

impl Timer {
    fn from_secs(seconds: u64) -> Self {
        Self {
            duration: Duration::from_secs(seconds),
            original_duration: Duration::from_secs(seconds),
            ..Self::default()
        }
    }

    fn from_duration(duration: Duration) -> Self {
        Self {
            duration,
            original_duration: duration,
            ..Self::default()
        }
    }

    fn shrink_duration(&mut self, ratio: f64) {
        self.duration = self.duration.mul_f64(ratio);
    }

    fn adjust_duration(&mut self, ratio: f64) {
        self.duration = self.original_duration.mul_f64(ratio);
    }

    fn start(&mut self) {
        self.start_instant = Instant::now();
        self.active = true;
    }

    fn pause(&mut self) {
        if self.active {
            self.elapsed += Instant::now() - self.start_instant;
            self.active = false;
        }
    }

    fn get_remaining(&self) -> Duration {
        match self.active {
            true => self
                .duration
                .saturating_sub(self.elapsed + (Instant::now() - self.start_instant)),
            false => self.duration.saturating_sub(self.elapsed),
        }
    }

    fn get_percentage(&self) -> f64 {
        self.get_remaining().as_secs() as f64 / self.duration.as_secs() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
