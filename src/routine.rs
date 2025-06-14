pub mod flex;
pub mod parse_routine;
pub mod task;

pub use task::{CompletionStatus, Task};

use std::cmp::max;
use std::time::Duration;

use chrono::{DateTime, Local};

use flex::{Flex, FlexItem};

#[derive(Debug, Copy, Clone)]
pub enum ToggleFailure {
    NoSelection,
    NoDoneToSkip,
}

impl FlexItem for Task {
    fn min_size(&self) -> Duration {
        self.elapsed
    }
    fn max_size(&self) -> Duration {
        match self.status {
            CompletionStatus::NotYet => max(self.elapsed, self.original_duration),
            _ => self.elapsed,
        }
    }
}

impl Flex for Routine {
    fn get_items(&self) -> &Vec<impl FlexItem> {
        &self.tasks
    }
}

// for testing, methods that use this timing should not query the current time but accept it as a
// value
#[derive(Default, Debug, Copy, Clone)]
enum TimeMode {
    #[default]
    ExpectedEnd,
    FixedEnd(DateTime<Local>),
}

#[derive(Default, Debug)]
pub struct Routine {
    /// An ordered list of the tasks.
    pub tasks: Vec<Task>,
    /// The active task, if any.
    /// TODO this should probably eventually use an ID number.
    //active: Option<usize>,
    /// Amount of time to try to fit tasks into.
    pub flex_goal: Duration,
    /// Timing Mode
    mode: TimeMode,
    /// Time elapsed while not not focused on a task
    spilled_time: Duration,
}

impl Routine {
    pub fn with_tasks(tasks: Vec<Task>) -> Self {
        let original_max = tasks
            .iter()
            .fold(Duration::ZERO, |acc, t| acc + t.original_duration);
        Self {
            tasks,
            //            active: match len {
            //              0 => None,
            //              _ => Some(0),
            //            },
            spilled_time: Duration::ZERO,
            flex_goal: original_max,
            mode: TimeMode::ExpectedEnd,
        }
    }

    // Not ideal to clone here but I'm only using it upon user input and routines
    // shouldn't be that long. Good enough for prototyping. TODO
    pub fn get_checkboxes(&self) -> Vec<bool> {
        self.tasks
            .iter()
            .map(|t| match t.status {
                // The purpose of this function is for auto cursor movement
                // to the next available task. Matching over the enum to
                // make sure any future status is considered.
                CompletionStatus::NotYet => true,
                CompletionStatus::Done => false,
                CompletionStatus::Skipped => false,
            })
            .collect()
    }

    fn sync_goal(&mut self, now: DateTime<Local>) {
        // TODO make this robust to timing glitches
        match self.mode {
            TimeMode::ExpectedEnd => (),
            TimeMode::FixedEnd(deadline) => {
                // TODO handle past due case
                let time_left = (deadline - now).to_std().expect("handle negative");
                let time_spent = self.elapsed();
                self.flex_goal = time_spent + time_left;
                self.update_flex();
            }
        }
    }

    pub fn set_deadline(&mut self, deadline: DateTime<Local>) {
        self.mode = TimeMode::FixedEnd(deadline);
        // TODO put timing call in app module
        self.sync_goal(Local::now());
    }

    fn increase_time_for_new_task(&mut self, task: &Task) {
        match self.mode {
            TimeMode::ExpectedEnd => self.flex_goal += task.original_duration,
            // want to take people's deadlines seriously and not accidentally
            // extend them
            TimeMode::FixedEnd(deadline) => (),
        }
    }

    pub fn push(&mut self, task: Task) {
        self.increase_time_for_new_task(&task);
        self.tasks.push(task);
        self.update_flex();
    }

    pub fn insert(&mut self, i: usize, task: Task) {
        self.increase_time_for_new_task(&task);
        self.tasks.insert(i, task);
        self.update_flex();
    }

    /*
    pub fn get_current(&mut self) -> Option<&mut Task> {
        match self.active {
            Some(i) => self.tasks.get_mut(i),
            None => None,
        }
    }
    */
    fn get_nth_mut(&mut self, active: Option<usize>) -> Option<&mut Task> {
        match active {
            Some(i) => self.tasks.get_mut(i),
            None => None,
        }
    }

    pub fn get_nth(&self, active: Option<usize>) -> Option<&Task> {
        match active {
            Some(i) => self.tasks.get(i),
            None => None,
        }
    }

    fn update_flex(&mut self) {
        let times = self
            .flex(self.flex_goal - self.spilled_time)
            .unwrap_or(vec![Duration::ZERO; self.tasks.len()]);
        times
            .iter()
            .zip(self.tasks.iter_mut())
            .for_each(|(&time, task)| {
                task.duration = time;
            });
    }

    pub fn toggle(&mut self, i: Option<usize>) -> Result<CompletionStatus, ToggleFailure> {
        if let Some(i) = self.get_nth_mut(i) {
            match i.status {
                CompletionStatus::Done => {
                    i.status = CompletionStatus::NotYet;
                    Ok(CompletionStatus::NotYet)
                }
                CompletionStatus::NotYet => {
                    i.status = CompletionStatus::Done;
                    self.update_flex();
                    Ok(CompletionStatus::Done)
                }
                CompletionStatus::Skipped => {
                    i.status = CompletionStatus::Done;
                    Ok(CompletionStatus::Done)
                }
            }
        } else {
            Err(ToggleFailure::NoSelection)
        }
    }

    pub fn skip(&mut self, i: Option<usize>) -> Result<CompletionStatus, ToggleFailure> {
        if let Some(i) = self.get_nth_mut(i) {
            match i.status {
                CompletionStatus::Skipped => {
                    i.status = CompletionStatus::NotYet;
                    Ok(CompletionStatus::NotYet)
                }
                CompletionStatus::NotYet => {
                    i.status = CompletionStatus::Skipped;
                    self.update_flex();
                    //self.next_no_wrap();
                    Ok(CompletionStatus::Skipped)
                }
                CompletionStatus::Done => Err(ToggleFailure::NoDoneToSkip),
            }
        } else {
            Err(ToggleFailure::NoSelection)
        }
    }

    pub fn duration(&self) -> Duration {
        self.tasks.iter().map(|task| task.duration).sum()
    }

    pub fn elapsed(&self) -> Duration {
        self.tasks.iter().map(|task| task.elapsed).sum()
    }

    pub fn total_originals(&self) -> Duration {
        self.tasks.iter().map(|task| task.original_duration).sum()
    }

    pub fn completed_originals(&self) -> Duration {
        self.tasks.iter().filter(|task| match task.status {
            CompletionStatus::NotYet => false,
            CompletionStatus::Done => true,
            // counting skipped tasks for percentage "complete" is a design
            // decision. It may end up being something to make configurable.
            // The benefit to true is more accurate timing and encouraging
            // making it through the list. The benefit to
            // false is gamification to discourage skipping. TODO
            CompletionStatus::Skipped => true,
        })
        .map(|task| task.original_duration)
        .sum()
    }

    pub fn remaining(&self) -> Duration {
        self.tasks.iter().map(|task| task.remaining()).sum()
    }

    pub fn elapse(&mut self, i: Option<usize>, duration: Duration) {
        match i {
            Some(i) => self.tasks[i].elapse(duration),
            None => self.spilled_time += duration,
        }
        self.update_flex();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO test about calculating duration for a whole list when something's completed
    // TODO dont crash when time elapsed exceeds total planned routine duration

    #[test]
    fn dont_shrink_ahead_of_schedule() {
        // tasks grow back to their original size if possible
        // when a task is completed ahead of schedule
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 80));
        list.push(Task::new("c", 60));

        list.elapse(Some(0), Duration::new(10, 0));
        list.toggle(Some(0)).unwrap();
        list.elapse(Some(1), Duration::new(100, 0));
        list.toggle(Some(1)).unwrap();

        assert_eq!(list.tasks[2].duration, Duration::new(60, 0));
    }

    #[test]
    fn grow_back() {
        // tasks grow back to their original size if possible
        // when a task is completed ahead of schedule
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 80));
        list.push(Task::new("c", 60));

        list.elapse(Some(0), Duration::new(140, 0));
        list.toggle(Some(0)).unwrap();
        list.toggle(Some(1)).unwrap();

        assert_eq!(list.tasks[2].duration, Duration::new(60, 0));
    }

    #[test]
    fn steady_list_long() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 80));
        list.push(Task::new("c", 10));
        list.push(Task::new("d", 60));
        let active = Some(0);

        let old_duration = list.duration();

        list.elapse(active, Duration::new(121, 0));
        list.elapse(active, Duration::new(10, 0));

        assert_eq!(list.duration(), old_duration);
    }

    #[test]
    fn steady_list_twice() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));
        let active = Some(0);

        let old_duration = list.duration();

        list.elapse(active, Duration::new(121, 0));
        list.elapse(active, Duration::new(10, 0));

        assert_eq!(list.duration(), old_duration);
    }

    #[test]
    fn duration_grows() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));
        let active = Some(0);

        list.elapse(active, Duration::new(121, 0));

        assert_eq!(list.tasks[0].duration, list.tasks[0].elapsed);
    }

    #[test]
    fn steady_list() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));
        let active = Some(0);

        let old_duration = list.duration();

        list.elapse(active, Duration::new(121, 0));

        assert_eq!(list.duration(), old_duration);
    }

    #[test]
    fn contract_task() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));
        let active = Some(0);

        list.elapse(active, Duration::new(121, 0));

        assert_eq!(list.tasks[1].duration, Duration::new(59, 0));
    }

    #[test]
    fn no_overflow() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));
        let active = Some(0);

        list.elapse(active, Duration::new(121, 0));

        assert_eq!(list.tasks[0].remaining(), Duration::ZERO);
    }

    #[test]
    fn step_time_list() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));
        let active = Some(0);

        list.elapse(active, Duration::new(1, 0));

        assert_eq!(list.tasks[0].elapsed, Duration::new(1, 0));
    }

    #[test]
    fn step_time() {
        let mut task = Task::new("a", 60);
        task.elapse(Duration::new(10, 0));
        task.elapse(Duration::new(5, 0));

        assert_eq!(task.elapsed, Duration::new(15, 0));
    }

    #[test]
    fn total_time_elapsed() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));

        list.tasks[0].elapse(Duration::new(10, 0));
        list.tasks[1].elapse(Duration::new(70, 0));

        assert_eq!(list.elapsed(), Duration::new(80, 0))
    }

    #[test]
    fn total_duration() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));

        assert_eq!(list.duration(), Duration::new(180, 0))
    }
}
