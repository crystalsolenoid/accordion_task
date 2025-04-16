use std::cmp::max;
use std::time::Duration;

use super::flex::{Flex, FlexItem};

#[derive(Debug)]
pub struct Task {
    /// How much time has already been spent on the task?
    pub elapsed: Duration,
    /// What was the original duration specified for the task?
    pub original_duration: Duration,
    /// Is the task completed?
    pub complete: bool,
    /// Name
    pub name: String,
    /// Current duration that may be shrunk
    pub duration: Duration,
}

impl FlexItem for Task {
    fn min_size(&self) -> Duration {
        self.elapsed
    }
    fn max_size(&self) -> Duration {
        match self.complete {
            false => max(self.elapsed, self.original_duration),
            true => self.elapsed,
        }
    }
}

impl Flex for Routine {
    fn get_items(&self) -> &Vec<impl FlexItem> {
        &self.tasks
    }
}

impl Task {
    pub fn new(name: &str, duration: u64) -> Self {
        Self {
            name: name.to_owned(),
            elapsed: Duration::ZERO,
            original_duration: Duration::new(duration, 0),
            duration: Duration::new(duration, 0),
            complete: false,
        }
    }

    pub fn remaining(&self) -> Duration {
        self.duration.saturating_sub(self.elapsed)
    }

    pub fn elapse(&mut self, duration: Duration) {
        self.elapsed += duration;
    }
}

#[derive(Default, Debug)]
pub struct Routine {
    /// An ordered list of the tasks.
    pub tasks: Vec<Task>,
    /// The active task, if any.
    /// TODO this should probably eventually use an ID number.
    pub active: Option<usize>,
    /// The original scheduled length of the list,
    /// which is used to know what to flex to.
    pub original_max: Duration,
}

impl Routine {
    pub fn with_tasks(tasks: Vec<Task>) -> Self {
        let len = tasks.len();
        let original_max = tasks
            .iter()
            .fold(Duration::ZERO, |acc, t| acc + t.original_duration);
        Self {
            tasks,
            active: match len {
                0 => None,
                _ => Some(0),
            },
            original_max,
        }
    }

    pub fn push(&mut self, task: Task) {
        self.original_max += task.original_duration;
        self.tasks.push(task);
    }

    pub fn get_current(&mut self) -> Option<&mut Task> {
        match self.active {
            Some(i) => self.tasks.get_mut(i),
            None => None,
        }
    }

    fn update_flex(&mut self) {
        let times = self.flex(self.original_max).expect("todo");
        times
            .iter()
            .zip(self.tasks.iter_mut())
            .for_each(|(&time, task)| {
                task.duration = time;
            });
    }

    pub fn toggle_current(&mut self) {
        if let Some(i) = self.get_current() {
            match i.complete {
                true => i.complete = false,
                false => {
                    i.complete = true;
                    self.update_flex();
                    self.next_no_wrap();
                }
            };
        };
    }

    pub fn next_no_wrap(&mut self) {
        if let Some(i) = self.active {
            if i < self.tasks.len() - 1 {
                self.next();
            }
        }
    }

    pub fn next(&mut self) {
        if let Some(i) = self.active {
            self.active = Some((i + 1) % self.tasks.len());
        }
    }

    pub fn previous(&mut self) {
        if let Some(i) = self.active {
            self.active = Some((i - 1) % self.tasks.len());
        }
    }

    pub fn duration(&self) -> Duration {
        self.tasks.iter().map(|task| task.duration).sum()
    }

    pub fn elapsed(&self) -> Duration {
        self.tasks.iter().map(|task| task.elapsed).sum()
    }

    pub fn remaining(&self) -> Duration {
        self.tasks.iter().map(|task| task.remaining()).sum()
    }

    pub fn elapse(&mut self, duration: Duration) {
        if let Some(i) = self.active {
            self.tasks[i].elapse(duration);
            self.update_flex();
        } // TODO else
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
        list.active = Some(0);

        list.elapse(Duration::new(10, 0));
        list.toggle_current();
        list.elapse(Duration::new(100, 0));
        list.toggle_current();

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
        list.active = Some(0);

        list.elapse(Duration::new(140, 0));
        list.toggle_current();
        list.toggle_current();

        assert_eq!(list.tasks[2].duration, Duration::new(60, 0));
    }

    #[test]
    fn steady_list_long() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 80));
        list.push(Task::new("c", 10));
        list.push(Task::new("d", 60));
        list.active = Some(0);

        let old_duration = list.duration();

        list.elapse(Duration::new(121, 0));
        list.elapse(Duration::new(10, 0));

        assert_eq!(list.duration(), old_duration);
    }

    #[test]
    fn steady_list_twice() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));
        list.active = Some(0);

        let old_duration = list.duration();

        list.elapse(Duration::new(121, 0));
        list.elapse(Duration::new(10, 0));

        assert_eq!(list.duration(), old_duration);
    }

    #[test]
    fn duration_grows() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));
        list.active = Some(0);

        list.elapse(Duration::new(121, 0));

        assert_eq!(list.tasks[0].duration, list.tasks[0].elapsed);
    }

    #[test]
    fn steady_list() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));
        list.active = Some(0);

        let old_duration = list.duration();

        list.elapse(Duration::new(121, 0));

        assert_eq!(list.duration(), old_duration);
    }

    #[test]
    fn contract_task() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));
        list.active = Some(0);

        list.elapse(Duration::new(121, 0));

        assert_eq!(list.tasks[1].duration, Duration::new(59, 0));
    }

    #[test]
    fn no_overflow() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));
        list.active = Some(0);

        list.elapse(Duration::new(121, 0));

        assert_eq!(list.tasks[0].remaining(), Duration::ZERO);
    }

    #[test]
    fn step_time_list() {
        let mut list = Routine::default();
        list.push(Task::new("a", 120));
        list.push(Task::new("b", 60));
        list.active = Some(0);

        list.elapse(Duration::new(1, 0));

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
