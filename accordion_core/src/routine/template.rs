use crate::routine::{Routine, Task};

#[cfg(feature = "web")]
use reactive_stores::Store;
#[cfg(feature = "web")]
use serde::{Deserialize, Serialize};

#[cfg_attr(
    feature = "web",
    derive(Store, Clone, Serialize, Deserialize, PartialEq, Eq)
)]
pub struct TaskTemplate {
    pub name: String,
    pub duration: u64,
    pub id: usize,
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
        // TODO assert unique IDs
        let counter = tasks.len();
        Self {
            name,
            tasks,
            counter,
        }
    }
    pub fn generate_routine(&self) -> Routine {
        let tasks = self.tasks.iter().map(|t| t.generate_task()).collect();
        Routine::with_tasks(tasks)
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
}

impl Default for RoutineTemplate {
    fn default() -> Self {
        Self {
            name: "Empty Routine".to_string(),
            tasks: vec![],
            counter: 0,
        }
    }
}
