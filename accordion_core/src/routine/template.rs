use crate::routine::{Routine, Task};

use chrono::{Local, NaiveTime};
#[cfg(feature = "web")]
use reactive_stores::Store;
use serde::Deserialize;
#[cfg(feature = "web")]
use serde::Serialize;

use super::TimeMode;

#[cfg_attr(
    feature = "web",
    derive(Store, Clone, Serialize, Deserialize, PartialEq, Eq)
)]
pub struct TaskTemplate {
    pub name: String,
    pub duration: u64,
    pub id: usize,
}

#[cfg_attr(feature = "web", derive(Store, Clone, Serialize, PartialEq, Eq))]
#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(rename = "deadline")]
    #[serde(default)]
    pub default_deadline: Option<NaiveTime>,
}

#[cfg_attr(
    feature = "web",
    derive(Store, Clone, Serialize, Deserialize, PartialEq, Eq)
)]
pub struct RoutineTemplate {
    pub name: String,
    #[cfg_attr(feature = "web", store(key: usize = |row| row.id))]
    pub tasks: Vec<TaskTemplate>,
    counter: usize,
    pub config: Config,
}

impl TaskTemplate {
    pub fn generate_task(&self) -> Task {
        Task::new(&self.name, self.duration, self.id)
    }

    pub fn new(name: &str, duration: u64, id: usize) -> Self {
        Self {
            name: name.to_string(),
            duration,
            id,
        }
    }
}

impl RoutineTemplate {
    pub fn new(name: String, tasks: Vec<TaskTemplate>) -> Self {
        // TODO assert unique sequential IDs?
        let counter = tasks.len();
        Self {
            name,
            tasks,
            counter,
            config: Config::default(),
        }
    }

    pub fn with_config(name: String, tasks: Vec<TaskTemplate>, config: Config) -> Self {
        // TODO assert unique sequential IDs?
        let counter = tasks.len();
        Self {
            name,
            tasks,
            counter,
            config,
        }
    }

    pub fn generate_routine(&self) -> Routine {
        let tasks = self.tasks.iter().map(|t| t.generate_task()).collect();
        let mut routine = Routine::with_tasks(tasks);

        // If a default deadline is specified, use it
        if let Some(deadline) = self.config.default_deadline {
            let now = Local::now();
            let next_deadline = crate::utils::interpret_naive_time(now, deadline);
            routine.set_deadline(next_deadline);
        }

        routine
    }

    pub fn push(&mut self, task: TaskTemplate) {
        let mut task = task;
        task.id = self.counter;
        self.tasks.push(task);
        self.counter += 1;
    }

    pub fn move_task_sooner(&mut self, i: usize) {
        self.tasks.swap(i, i.saturating_sub(1));
    }

    pub fn move_task_later(&mut self, i: usize) {
        if self.tasks.get(i + 1).is_some() {
            self.tasks.swap(i, i + 1);
        }
    }

    pub fn remove_task(&mut self, i: usize) {
        if self.tasks.get(i).is_some() {
            self.tasks.remove(i);
        }
    }

    pub fn total_duration(&self) -> u64 {
        self.tasks.iter().map(|t| t.duration).sum()
    }
}

impl Default for RoutineTemplate {
    fn default() -> Self {
        Self {
            name: "Empty Routine".to_string(),
            tasks: vec![],
            counter: 0,
            config: Config::default(),
        }
    }
}
