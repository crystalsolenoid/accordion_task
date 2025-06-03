pub mod list_pointer;
mod logging;

use crate::cli::Cli;
use crate::routine::{
    parse_routine,
    task::{parse_new_task, CompletionStatus, Task},
    Routine,
};
use list_pointer::ListPointer;
use logging::{LogElement, RoutineLogger};

use chrono::{DateTime, Days, Local, MappedLocalTime};
use std::time::{Duration, Instant};
use tui_textarea::TextArea;

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
    pub menu_focus: Mode,
    pub text_input: TextArea<'static>,
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
            text_input: TextArea::default(),
            menu_focus: Mode::Navigation,
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

        /*
        app.task_widget_state.select(
            match app.tasks.len() {
                0 => None,
                _ => Some(0),
            });
        */
        //app.task_widget_state.select(app.tasks.active).unwrap();

        app
    }

    pub fn get_current_task_name(&self) -> Option<&str> {
        let i = self.task_widget_state.selected();
        self.tasks.get_nth(i)
            .map(|t| t.name.as_str())
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
        self.tasks.elapse(self.task_widget_state.selected(), delta);
        if let Some(t) = self.tasks.get_nth(self.task_widget_state.selected()) {
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

    pub fn append_task_start(&mut self) {
        self.menu_focus = Mode::Typing(Menu::AppendTask);
    }

    pub fn insert_task_start(&mut self) {
        self.menu_focus = Mode::Typing(Menu::InsertTask);
    }

    pub fn pause(&mut self) {
        self.task_widget_state.pause();
        self.menu_focus = Mode::Typing(Menu::Pause);
    }

    fn append_task_submit(&mut self) {
        let name = self.text_input.lines()[0].to_owned();
        let task = parse_new_task(&name);
        self.task_widget_state.append_item();
        self.tasks.push(task);
    }

    fn insert_task_submit(&mut self) {
        let name = self.text_input.lines()[0].to_owned();
        // TODO fix ownership of name
        let task = parse_new_task(&name);
        self.task_widget_state.append_item();
        let i = self.task_widget_state.selected().unwrap_or(0) + 1;
        self.tasks.insert(i, task);
    }

    fn unpause(&mut self) {
        // TODO pausing should log the
        // duration of the pause,
        // the fact that it was a pause,
        // and the message.
        let message = self.text_input.lines()[0].to_owned();
        self.logger.log_comment(&message, Local::now());
        self.task_widget_state.unpause();
    }

    pub fn cancel_typing(&mut self) {
        self.text_input = TextArea::default();
        self.menu_focus = Mode::Navigation;
    }

    pub fn submit_typing(&mut self, menu: Menu) {
        match menu {
            Menu::AppendTask => self.append_task_submit(),
            Menu::InsertTask => self.insert_task_submit(),
            Menu::Pause => self.unpause(),
        }
        self.cancel_typing();
    }

    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }

    pub fn toggle_help(&mut self) {
        self.help_menu = !self.help_menu;
    }

    pub fn attempt_toggle(&mut self) {
        let i = self.task_widget_state.selected();
        match self.tasks.toggle(i) {
            Ok(CompletionStatus::Done) => {
                let task = self
                    .tasks
                    .get_nth(i)
                    .expect("this should always exist here");
                self.logger.log(LogElement::completed(task));
                self.bouncing_next_task();
            }
            Ok(CompletionStatus::NotYet) => {
                let task = self
                    .tasks
                    .get_nth(self.task_widget_state.selected())
                    .expect("this should always exist here");
                self.logger.log(LogElement::uncompleted(task));
            }
            Err(_) => (),
            Ok(CompletionStatus::Skipped) => panic!("this should never happen?"),
        };
    }

    pub fn attempt_skip(&mut self) {
        let i = self.task_widget_state.selected();
        match self.tasks.skip(i) {
            Ok(CompletionStatus::Skipped) => {
                let task = self
                    .tasks
                    .get_nth(i)
                    .expect("this should always exist here");
                self.logger.log(LogElement::skipped(task));
                self.bouncing_next_task();
            }
            Ok(CompletionStatus::NotYet) => {
                let task = self
                    .tasks
                    .get_nth(self.task_widget_state.selected())
                    .expect("this should always exist here");
                self.logger.log(LogElement::unskipped(task));
            }
            Err(_) => (),
            Ok(CompletionStatus::Done) => panic!("this should never happen?"),
        };
    }

    pub fn next_task(&mut self) {
        let _ = self.task_widget_state.try_next();
    }

    fn bouncing_next_task(&mut self) {
        let selectable = self.tasks.get_checkboxes().into_iter();
        match self
            .task_widget_state
            .try_next_selectable(selectable.clone())
        {
            // TODO shouldnt have to clone here
            Ok(_) => (),
            Err(_) => {
                let _ = self.task_widget_state.try_prev_selectable(selectable);
            }
        }
    }

    pub fn next_available_task(&mut self) {
        let selectable = self.tasks.get_checkboxes().into_iter();
        let _ = self.task_widget_state.try_next_selectable(selectable);
    }

    pub fn prev_task(&mut self) {
        let _ = self.task_widget_state.try_prev();
    }

    pub fn prev_available_task(&mut self) {
        let selectable = self.tasks.get_checkboxes().into_iter();
        let _ = self.task_widget_state.try_prev_selectable(selectable);
    }
}

#[derive(Debug)]
pub enum Mode {
    Navigation,
    Typing(Menu),
}

#[derive(Debug, Copy, Clone)]
pub enum Menu {
    AppendTask,
    InsertTask,
    Pause,
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO
}
