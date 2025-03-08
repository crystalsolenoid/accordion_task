use std::time::Duration;

#[derive(Debug)]
pub struct StaticTask {
    /// How much time has already been spent on the task?
    pub elapsed: Duration,
    /// What was the original duration specified for the task?
    pub duration: Duration,
    /// Is the task completed?
    pub complete: bool,
    /// Name
    pub name: String,
}

impl StaticTask {
    pub fn new(name: &str, duration: u64) -> Self {
        Self {
            name: name.to_owned(),
            elapsed: Duration::ZERO,
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

    pub fn from_secs(seconds: u64) -> Self {
        Self::new("unknown", seconds)
    }
}

#[derive(Default, Debug)]
pub struct StaticTaskList {
    /// An ordered list of the tasks.
    pub tasks: Vec<StaticTask>,
    /// The active task, if any.
    /// TODO this should probably eventually use an ID number.
    pub active: Option<usize>,
}

impl StaticTaskList {
    pub fn with_tasks<'a>(tasks: Vec<StaticTask>) -> Self {
        let len = tasks.len();
        Self {
            tasks,
            active: match len {
                0 => None,
                _ => Some(0),
            }
        }
    }

    pub fn get_current(&mut self) -> Option<&mut StaticTask> {
        match self.active {
            Some(i) => self.tasks.get_mut(i),
            None => None,
        }
    }

    pub fn toggle_current(&mut self) {
        if let Some(i) = self.get_current() {
            match i.complete {
                true => i.complete = false,
                false => {
                    i.complete = true;
                    self.next_no_wrap();
                }
            };
        };
    }

    pub fn next_no_wrap(&mut self) {
        let i = match self.active {
            Some(i) => {
                if i < self.tasks.len() - 1 {
                    self.next()
                }
            }
            None => (),
        };
    }

    pub fn next(&mut self) {
        if let Some(i) = self.active {
            self.active = Some((i + 1) % self.tasks.len());
        }
    }

    pub fn previous(&mut self) {
        todo!();
        if let Some(i) = self.active {
            self.active = Some((i + 1) % self.tasks.len());
        }
    }

    pub fn duration(&self) -> Duration {
        self.tasks.iter().map(|task| task.duration).sum()
    }

    pub fn elapsed(&self) -> Duration {
        self.tasks.iter().map(|task| task.elapsed).sum()
    }

    pub fn elapse(&mut self, duration: Duration) {
        if let Some(i) = self.active {
            self.tasks[i].elapse(duration);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_time_list() {
        let mut list = StaticTaskList::default();
        list.tasks.push(StaticTask::new("a", 120));
        list.tasks.push(StaticTask::new("b", 60));
        list.active = Some(0);

        list.elapse(Duration::new(1, 0));

        assert_eq!(list.tasks[0].elapsed, Duration::new(1, 0));
    }

    #[test]
    fn step_time() {
        let mut task = StaticTask::new("a", 60);
        task.elapse(Duration::new(10, 0));
        task.elapse(Duration::new(5, 0));

        assert_eq!(task.elapsed, Duration::new(15, 0));
    }

    #[test]
    fn total_time_elapsed() {
        let mut list = StaticTaskList::default();
        list.tasks.push(StaticTask::new("a", 120));
        list.tasks.push(StaticTask::new("b", 60));

        list.tasks[0].elapse(Duration::new(10, 0));
        list.tasks[1].elapse(Duration::new(70, 0));

        assert_eq!(list.elapsed(), Duration::new(80, 0))
    }

    #[test]
    fn total_duration() {
        let mut list = StaticTaskList::default();
        list.tasks.push(StaticTask::new("a", 120));
        list.tasks.push(StaticTask::new("b", 60));

        assert_eq!(list.duration(), Duration::new(180, 0))
    }
}
