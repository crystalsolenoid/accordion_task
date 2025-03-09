use std::time::Duration;
use std::cmp::max;

#[derive(Debug)]
pub struct StaticTask {
    /// How much time has already been spent on the task?
    pub elapsed: Duration,
    /// What was the original duration specified for the task?
    pub original_duration: Duration,
    /// Is the task completed?
    pub complete: bool,
    /// Name
    pub name: String,
    pub shrink_ratio: f64,
}

impl StaticTask {
    pub fn new(name: &str, duration: u64) -> Self {
        Self {
            name: name.to_owned(),
            elapsed: Duration::ZERO,
            original_duration: Duration::new(duration, 0),
            shrink_ratio: 1.0,
            complete: false,
        }
    }

    pub fn duration(&self) -> Duration {
        max(self.original_duration.mul_f64(self.shrink_ratio), self.elapsed)
    }

    pub fn remaining(&self) -> Duration {
        self.duration().saturating_sub(self.elapsed)
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
        self.tasks.iter().map(|task| task.duration()).sum()
    }

    pub fn elapsed(&self) -> Duration {
        self.tasks.iter().map(|task| task.elapsed).sum()
    }

    pub fn elapse(&mut self, duration: Duration) {
        if let Some(i) = self.active {
            let old_duration = self.tasks[i].duration();
            self.tasks[i].elapse(duration);
            let new_duration = self.tasks[i].duration();
            if new_duration > old_duration {
                let overtime = new_duration - old_duration;
                let inactive_undone_duration = self.tasks.iter()
                    .enumerate()
                    .filter(|(j, task)| match (j, task.complete) {
                        (&j, _) if j == i => false,
                        (_, false) => true,
                        (_, true) => false,
                    })
                    .map(|(_, task)| task.duration())
                    .sum();
                // TODO use one iterator for both tasks
                let shrink_ratio = overtime.div_duration_f64(inactive_undone_duration);
                self.tasks.iter_mut()
                    .enumerate()
                    .filter(|(j, task)| match (j, task.complete) {
                        (&j, _) if j == i => false,
                        (_, false) => true,
                        (_, true) => false,
                    })
                    .for_each(|(_, task)| task.shrink_ratio *= (1.0 - shrink_ratio));
                // TODO I'm not sure yet if this works for subsequent elapse calls
            }
        } // TODO else
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO test about calculating duration for a whole list when something's completed

    #[test]
    fn grow_back() {
        // tasks grow back to their original size if possible
        // when a task is completed ahead of schedule
        let mut list = StaticTaskList::default();
        list.tasks.push(StaticTask::new("a", 120));
        list.tasks.push(StaticTask::new("b", 80));
        list.tasks.push(StaticTask::new("c", 60));
        list.active = Some(0);

        list.elapse(Duration::new(140, 0));
        list.toggle_current();
        list.toggle_current();

        assert_eq!(list.tasks[2].duration(), Duration::new(60, 0));
    }

    #[test]
    fn steady_list_long() {
        let mut list = StaticTaskList::default();
        list.tasks.push(StaticTask::new("a", 120));
        list.tasks.push(StaticTask::new("b", 80));
        list.tasks.push(StaticTask::new("c", 10));
        list.tasks.push(StaticTask::new("d", 60));
        list.active = Some(0);

        let old_duration = list.duration();

        list.elapse(Duration::new(121, 0));
        list.elapse(Duration::new(10, 0));

        assert_eq!(list.duration(), old_duration);
    }

    #[test]
    fn steady_list_twice() {
        let mut list = StaticTaskList::default();
        list.tasks.push(StaticTask::new("a", 120));
        list.tasks.push(StaticTask::new("b", 60));
        list.active = Some(0);

        let old_duration = list.duration();

        list.elapse(Duration::new(121, 0));
        list.elapse(Duration::new(10, 0));

        assert_eq!(list.duration(), old_duration);
    }

    #[test]
    fn duration_grows() {
        let mut list = StaticTaskList::default();
        list.tasks.push(StaticTask::new("a", 120));
        list.tasks.push(StaticTask::new("b", 60));
        list.active = Some(0);

        list.elapse(Duration::new(121, 0));

        assert_eq!(list.tasks[0].duration(), list.tasks[0].elapsed);
    }

    #[test]
    fn steady_list() {
        let mut list = StaticTaskList::default();
        list.tasks.push(StaticTask::new("a", 120));
        list.tasks.push(StaticTask::new("b", 60));
        list.active = Some(0);

        let old_duration = list.duration();

        list.elapse(Duration::new(121, 0));

        assert_eq!(list.duration(), old_duration);
    }
    
    #[test]
    fn contract_task() {
        let mut list = StaticTaskList::default();
        list.tasks.push(StaticTask::new("a", 120));
        list.tasks.push(StaticTask::new("b", 60));
        list.active = Some(0);

        list.elapse(Duration::new(121, 0));

        assert_eq!(list.tasks[1].duration(), Duration::new(59, 0));
    }
    
    #[test]
    fn no_overflow() {
        let mut list = StaticTaskList::default();
        list.tasks.push(StaticTask::new("a", 120));
        list.tasks.push(StaticTask::new("b", 60));
        list.active = Some(0);

        list.elapse(Duration::new(121, 0));

        assert_eq!(list.tasks[0].remaining(), Duration::ZERO);
    }

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
