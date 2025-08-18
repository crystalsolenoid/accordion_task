use crate::routine::{Routine, Task};

#[cfg(feature = "web")]
use reactive_stores::Store;
#[cfg(feature = "web")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "web", derive(Store, Clone, Serialize, Deserialize))]
pub struct TaskTemplate {
    pub name: String,
    pub duration: u64,
}

#[cfg_attr(feature = "web", derive(Store, Clone, Serialize, Deserialize))]
pub struct RoutineTemplate {
    pub name: String,
    #[cfg_attr(feature = "web", store(key: String = |row| row.name.clone()))]
    pub tasks: Vec<TaskTemplate>,
}

impl TaskTemplate {
    pub fn generate_task(&self) -> Task {
        Task::new(&self.name, self.duration)
    }

    pub fn new(name: &str, duration: u64) -> Self {
        Self {
            name: name.to_string(),
            duration,
        }
    }
}

impl RoutineTemplate {
    pub fn generate_routine(&self) -> Routine {
        let tasks = self.tasks.iter().map(|t| t.generate_task()).collect();
        Routine::with_tasks(tasks)
    }

    pub fn push(&mut self, task: TaskTemplate) {
        self.tasks.push(task);
    }
}

impl Default for RoutineTemplate {
    fn default() -> Self {
        Self {
            name: "Empty Routine".to_string(),
            tasks: vec![],
        }
    }
}
