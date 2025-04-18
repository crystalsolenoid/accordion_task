use ratatui::widgets::TableState;

mod flex;
mod parse_routine;
pub mod static_task;
mod logging;

use static_task::{Routine, Task};
use logging::{RoutineLogger, LogElement};

use chrono::{DateTime, Local};
use std::cmp::Ordering;
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
    pub tasks: Routine,
    /// routine timer
    pub routine_timer: Timer,
    pub last_tick: Instant,
    logger: RoutineLogger,
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
        let routine_name = parse_routine::get_routine_name().expect("Failed to load routine file");
        let tasks = Routine::with_tasks(
            parse_routine::read_csv().expect("Failed to load routine file"),
        );
        let logger = RoutineLogger::new(&tasks, &Local::now(), routine_name);
        let mut app = Self {
            should_quit: false,
            debug: false,
            logger,
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
        let remaining = self.routine_timer.get_remaining();
        let to_do = self.get_total_remaining();
        match to_do.cmp(&remaining) {
            Ordering::Greater => SignedDuration::DEFICIT(to_do - remaining),
            Ordering::Less => SignedDuration::SURPLUS(remaining - to_do),
            Ordering::Equal => SignedDuration::ZERO,
        }
    }

    pub fn get_total_remaining(&self) -> Duration {
        self.tasks.remaining()
        /*
        self.tasks
            .tasks
            .iter()
            .filter(|task| !task.complete)
            .map(|task| task.remaining())
            .sum()
        */
    }

    pub fn get_total_duration(&self) -> Duration {
        self.tasks.duration()
        /*
        self.tasks
            .tasks
            .iter()
            .filter(|task| !task.complete)
            .map(|task| task.duration)
            .sum()
        */
    }

    pub fn get_unused_time(&self) -> Duration {
        todo!() //remove this function?
                /*
                self.tasks
                    .tasks
                    .iter()
                    .filter(|task| task.complete)
                    .map(|task| task.remaining())
                    .sum()
                */
    }

    pub fn get_start_time(&self) -> DateTime<Local> {
        self.routine_timer.start_time
    }

    pub fn get_projected_end_time(&self) -> DateTime<Local> {
        Local::now() + self.get_total_remaining()
    }

    pub fn start_routine(&mut self) {
        self.routine_timer.start();
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        let this_tick = Instant::now();
        let delta = this_tick - self.last_tick;
        self.last_tick = this_tick;

        cli_log::debug!("Tick");
        self.tasks.elapse(delta);
        if let Some(t) = self.tasks.get_current() {
            self.logger.log(LogElement::elapsed(&t, delta));
        }
    }

    pub fn get_time_elapsed(&self) -> Duration {
        self.routine_timer.start_instant.elapsed()
    }

    pub fn get_percentage_elapsed(&self) -> f64 {
        self.routine_timer.get_percentage()
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.logger.finish();
        self.should_quit = true;
    }

    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }

    pub fn attempt_toggle(&mut self) {
        if let Some(task) = self.tasks.get_current() {
            self.logger.log(LogElement::completed(&task));
        }
        self.tasks.toggle_current();
        self.task_widget_state.select(self.tasks.active);
    }

    pub fn attempt_skip(&mut self) {
        if let Some(task) = self.tasks.get_current() {
            // TODO for this and complete log, make sure unmarking them generates the correct log
            // entry
            self.logger.log(LogElement::skipped(&task));
        }
        self.tasks.skip_current();
        self.task_widget_state.select(self.tasks.active);
    }

    pub fn next_task(&mut self) {
        cli_log::debug!("Next task");
        // TODO update version of ratatui
        //        self.task_widget_state.select_next();
        self.task_widget_state.select_next();
        self.tasks.active = self.task_widget_state.selected();
        /*
        if let Some(i) = self.task_widget_state.selected() {
            let j = if i >= self.tasks.tasks.len() - 1 {
                0
            } else {
                i + 1
            };
            self.task_widget_state.select(Some(j));
            self.tasks.active = self.task_widget_state.selected();
        }
        */
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

impl Default for App {
    fn default() -> Self {
        Self::new()
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

    fn start(&mut self) {
        self.start_instant = Instant::now();
        self.active = true;
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
    // TODO
}
