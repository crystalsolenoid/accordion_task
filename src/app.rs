mod flex;
mod list_pointer;
mod logging;
mod parse_routine;
pub mod static_task;

use crate::cli::Cli;
use list_pointer::ListPointer;
use logging::{LogElement, RoutineLogger};
use static_task::{CompletionStatus, Routine, Task};

use chrono::{DateTime, Days, Local, MappedLocalTime};
use std::time::{Duration, Instant};

/// Application.
#[derive(Debug)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    pub debug: bool,
    pub help_menu: bool,
    /// counter
    pub counter: i64,
    /// task display widget
    pub task_widget_state: ListPointer,
    /// task internal list
    pub tasks: Routine,
    /// routine timer
    pub last_tick: Instant,
    logger: RoutineLogger,
    pub start_time: DateTime<Local>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(cli: Cli) -> App {
        let routine_name = cli.routine_path;
        let tasks =
            Routine::with_tasks(parse_routine::read_csv().expect("Failed to load routine file"));
        let length = tasks.tasks.len();
        let logger = RoutineLogger::new(&tasks, &Local::now(), routine_name);
        let mut app = Self {
            start_time: Local::now(),
            should_quit: false,
            debug: false,
            help_menu: false,
            logger,
            counter: 0,
            tasks,
            task_widget_state: ListPointer::new(length),
            last_tick: Instant::now(),
        };

        let now = Local::now();
        if let Some(deadline) = cli.deadline {
            // TODO handle DST
            let today_deadline = match now.with_time(deadline) {
                MappedLocalTime::Single(t) => t,
                _ => todo!(), // Risks crash around DST change
            };
            let deadline = if today_deadline < now {
                let tomorrow = match now.checked_add_days(Days::new(1)) {
                    Some(t) => t,
                    None => todo!(), // Risks DST crash
                };
                match tomorrow.with_time(deadline) {
                    MappedLocalTime::Single(t) => t,
                    _ => todo!(), // Risks crash around DST change
                }
            } else {
                today_deadline
            };
            app.tasks.set_deadline(deadline);
        };

        app.task_widget_state.select(app.tasks.active).unwrap();

        app
    }

    pub fn get_total_remaining(&self) -> Duration {
        self.tasks.remaining()
    }

    pub fn get_total_duration(&self) -> Duration {
        self.tasks.duration()
    }

    pub fn get_start_time(&self) -> DateTime<Local> {
        self.start_time
    }

    pub fn get_projected_end_time(&self) -> DateTime<Local> {
        Local::now() + self.get_total_remaining()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        let this_tick = Instant::now();
        let delta = this_tick - self.last_tick;
        self.last_tick = this_tick;

        cli_log::debug!("Tick");
        self.tasks.elapse(delta);
        if let Some(t) = self.tasks.get_current() {
            self.logger.log(LogElement::elapsed(t, delta));
        }
    }

    pub fn get_time_elapsed(&self) -> Duration {
        self.tasks.elapsed()
    }

    pub fn get_percentage_elapsed(&self) -> f64 {
        //0.5 // TODO
        1.0 - self
            .tasks
            .elapsed()
            .div_duration_f64(self.tasks.elapsed() + self.tasks.remaining())
        // self.routine_timer.get_percentage()
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.logger.finish();
        self.should_quit = true;
    }

    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }

    pub fn toggle_help(&mut self) {
        self.help_menu = !self.help_menu;
    }

    pub fn attempt_toggle(&mut self) {
        match self.tasks.toggle_current() {
            Ok(CompletionStatus::Done) => {
                let task = self
                    .tasks
                    .get_current()
                    .expect("this should always exist here");
                self.logger.log(LogElement::completed(task));
                let _ = self.task_widget_state.try_next();
                self.tasks.active = self.task_widget_state.selected();
            }
            Ok(CompletionStatus::NotYet) => {
                let task = self
                    .tasks
                    .get_current()
                    .expect("this should always exist here");
                self.logger.log(LogElement::uncompleted(task));
            }
            Err(_) => (),
            Ok(CompletionStatus::Skipped) => panic!("this should never happen?"),
        };
    }

    pub fn attempt_skip(&mut self) {
        match self.tasks.skip_current() {
            Ok(CompletionStatus::Skipped) => {
                let task = self
                    .tasks
                    .get_current()
                    .expect("this should always exist here");
                self.logger.log(LogElement::skipped(task));
                let _ = self.task_widget_state.try_next();
                self.tasks.active = self.task_widget_state.selected();
            }
            Ok(CompletionStatus::NotYet) => {
                let task = self
                    .tasks
                    .get_current()
                    .expect("this should always exist here");
                self.logger.log(LogElement::unskipped(task));
            }
            Err(_) => (),
            Ok(CompletionStatus::Done) => panic!("this should never happen?"),
        };
    }

    pub fn next_task(&mut self) {
        let _ = self.task_widget_state.try_next();
        self.tasks.active = self.task_widget_state.selected();
    }

    pub fn prev_task(&mut self) {
        // TODO update version of ratatui
        //        self.task_widget_state.select_previous();
        let _ = self.task_widget_state.try_prev();
        self.tasks.active = self.task_widget_state.selected();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO
}
