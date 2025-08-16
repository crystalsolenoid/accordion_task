mod logging;

use crate::cli::{self, Cli};
use crate::config::{self, Config};
use accordion_core::routine::{
    Routine,
    task::{self, CompletionStatus, Task},
};
use accordion_core::session::Session;
use logging::{LogElement, RoutineLogger};

use chrono::{DateTime, Local};
use color_eyre::{
    Result,
    eyre::{OptionExt, WrapErr},
};
use std::time::Duration;
use tui_textarea::TextArea;

/// Application.
pub struct App {
    pub config: Config,
    /// should the application exit?
    pub should_quit: bool,
    pub debug: bool,
    pub help_menu: bool,
    /// task display widget
    pub session: Session,
    logger: RoutineLogger,
    pub menu_focus: Mode,
    pub text_input: TextArea<'static>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(cli: Cli) -> Result<App> {
        let routine_name = cli
            .routine_path
            .ok_or_eyre("Routine launcher not yet implemented. Please specify a routine path.")?;
        let tasks =
            Routine::with_tasks(cli::get_routine().wrap_err("Failed to load routine file")?);
        let logger = RoutineLogger::new(&tasks, &Local::now(), &routine_name)
            .wrap_err("Logger failed to initialize.")?;
        let mut app = Self {
            config: config::load(),
            text_input: TextArea::default(),
            menu_focus: Mode::Navigation,
            should_quit: false,
            session: Session::new(tasks),
            debug: false,
            help_menu: false,
            logger,
        };

        let now = Local::now();
        if let Some(deadline) = cli.deadline {
            let deadline = accordion_core::utils::interpret_naive_time(now, deadline);
            app.session.tasks.set_deadline(deadline);
        };

        /*
        app.task_widget_state.select(
            match app.tasks.len() {
                0 => None,
                _ => Some(0),
            });
        */
        //app.task_widget_state.select(app.tasks.active).unwrap();

        Ok(app)
    }

    pub fn get_current_task_name(&self) -> Option<&str> {
        let i = self.session.selected.selected();
        self.session.tasks.get_nth(i).map(|t| t.name.as_str())
    }

    pub fn get_total_remaining(&self) -> Duration {
        self.session.tasks.remaining()
    }

    pub fn get_total_duration(&self) -> Duration {
        self.session.tasks.duration()
    }

    pub fn get_start_time(&self) -> DateTime<Local> {
        self.session.start_time
    }

    pub fn get_projected_end_time(&self) -> DateTime<Local> {
        Local::now() + self.get_total_remaining()
    }

    pub fn tick(&mut self) {
        let delta = self.session.tick().to_std().unwrap();
        if let Some(t) = self.session.tasks.get_nth(self.session.selected.selected()) {
            self.logger.log(LogElement::elapsed(t, delta));
        }
    }

    pub fn get_time_elapsed(&self) -> Duration {
        self.session.tasks.elapsed()
    }

    pub fn get_percentage_elapsed(&self) -> f64 {
        self.session.get_percentage_elapsed()
    }

    /// Set `should_quit` to `true` to quit the application.
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
        self.session.selected.pause();
        self.menu_focus = Mode::Typing(Menu::Pause);
    }

    fn append_task_submit(&mut self) {
        let name = self.text_input.lines()[0].clone();
        let task = task::parse_new(&name);
        self.session.selected.append_item();
        self.session.tasks.push(task);
    }

    fn insert_task_submit(&mut self) {
        let name = self.text_input.lines()[0].clone();
        // TODO fix ownership of name
        let task = task::parse_new(&name);
        self.session.selected.append_item();
        let i = self.session.selected.selected().unwrap_or(0) + 1;
        self.session.tasks.insert(i, task);
    }

    fn unpause(&mut self) {
        // TODO pausing should log the
        // duration of the pause,
        // the fact that it was a pause,
        // and the message.
        let message = self.text_input.lines()[0].clone();
        self.logger.log_comment(&message, Local::now());
        self.session.selected.unpause();
    }

    pub fn cancel_typing(&mut self, menu: Menu) {
        self.text_input = TextArea::default();
        self.menu_focus = Mode::Navigation;
        if menu == Menu::Pause {
            // TODO duplicates work. refactor?
            self.session.selected.unpause();
        }
    }

    pub fn submit_typing(&mut self, menu: Menu) {
        match menu {
            Menu::AppendTask => self.append_task_submit(),
            Menu::InsertTask => self.insert_task_submit(),
            Menu::Pause => self.unpause(),
        }
        self.cancel_typing(menu);
    }

    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }

    pub fn toggle_help(&mut self) {
        self.help_menu = !self.help_menu;
    }

    pub fn attempt_toggle(&mut self) {
        let i = self.session.selected.selected();
        match self.session.toggle() {
            Ok(CompletionStatus::Done) => {
                let task = self
                    .session
                    .tasks
                    .get_nth(i)
                    .expect("this should always exist here");
                self.logger.log(LogElement::completed(task));
                self.bouncing_next_task();
            }
            Ok(CompletionStatus::NotYet) => {
                let task = self
                    .session
                    .tasks
                    .get_nth(self.session.selected.selected())
                    .expect("this should always exist here");
                self.logger.log(LogElement::uncompleted(task));
            }
            Err(_) => (),
            Ok(CompletionStatus::Skipped) => panic!("this should never happen?"),
        };
    }

    pub fn attempt_skip(&mut self) {
        let i = self.session.selected.selected();
        match self.session.tasks.skip(i) {
            Ok(CompletionStatus::Skipped) => {
                let task = self
                    .session
                    .tasks
                    .get_nth(i)
                    .expect("this should always exist here");
                self.logger.log(LogElement::skipped(task));
                self.bouncing_next_task();
            }
            Ok(CompletionStatus::NotYet) => {
                let task = self
                    .session
                    .tasks
                    .get_nth(self.session.selected.selected())
                    .expect("this should always exist here");
                self.logger.log(LogElement::unskipped(task));
            }
            Err(_) => (),
            Ok(CompletionStatus::Done) => panic!("this should never happen?"),
        };
    }

    pub fn next_task(&mut self) {
        let _ = self.session.selected.try_next();
    }

    fn bouncing_next_task(&mut self) {
        let selectable = self.session.tasks.get_checkboxes().into_iter();
        match self
            .session
            .selected
            .try_next_selectable(selectable.clone())
        {
            // TODO shouldnt have to clone here
            Ok(()) => (),
            Err(_) => {
                let _ = self.session.selected.try_prev_selectable(selectable);
            }
        }
    }

    pub fn next_available_task(&mut self) {
        let selectable = self.session.tasks.get_checkboxes().into_iter();
        let _ = self.session.selected.try_next_selectable(selectable);
    }

    pub fn prev_task(&mut self) {
        let _ = self.session.selected.try_prev();
    }

    pub fn prev_available_task(&mut self) {
        let selectable = self.session.tasks.get_checkboxes().into_iter();
        let _ = self.session.selected.try_prev_selectable(selectable);
    }
}

#[derive(Debug)]
pub enum Mode {
    Navigation,
    Typing(Menu),
}

#[derive(PartialEq, Debug, Copy, Clone)]
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
