use std::time::Duration;

use crate::{
	routine::{Routine, Task},
	utils::format_duration,
};

use chrono::{Local, NaiveTime};
#[cfg(feature = "web")]
use reactive_stores::Store;
use serde::Deserialize;
use serde::Serialize;

#[cfg_attr(
	feature = "web",
	derive(Store, Clone, Serialize, Deserialize, PartialEq, Eq)
)]
pub struct TaskTemplate {
	pub name: String,
	pub duration: u64,
	pub id: usize,
}

#[cfg_attr(feature = "web", derive(Store, PartialEq, Eq))]
#[derive(Clone, Deserialize, Serialize, Default)]
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
	pub config: Option<Config>,
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
			config: None,
		}
	}

	pub fn with_config(name: String, tasks: Vec<TaskTemplate>, config: Option<Config>) -> Self {
		// TODO assert unique sequential IDs?
		let counter = tasks.len();
		Self {
			name,
			tasks,
			counter,
			config,
		}
	}

	pub fn get_routine_file(&self) -> String {
		let config = if let Some(c) = &self.config {
			let string_config = toml::to_string(&c).unwrap();
			format!("---\n{}---\n", string_config)
		} else {
			String::new()
		};
		let mut wrtr = csv::WriterBuilder::new()
			.delimiter(b',')
			.comment(Some(b'#'))
			.from_writer(vec![]);
		wrtr.write_record(["task", "duration"]).unwrap();
		self.tasks.iter().for_each(|task| {
			wrtr.write_record(&[
				task.name.clone(),
				format_duration(Duration::from_secs(task.duration)),
			])
			.unwrap();
		});
		let tasks = String::from_utf8(wrtr.into_inner().unwrap()).unwrap();
		config.to_owned() + &tasks
	}

	pub fn generate_routine(&self) -> Routine {
		let tasks = self.tasks.iter().map(|t| t.generate_task()).collect();

		let mut routine = Routine::with_tasks(tasks);

		if let Some(config) = &self.config {
			// If a default deadline is specified, use it
			if let Some(deadline) = config.default_deadline {
				let now = Local::now();
				let next_deadline = crate::utils::interpret_naive_time(now, deadline);
				routine.set_deadline(next_deadline);
			}
		}

		routine
	}

	pub fn conf_set_default_deadline(&mut self, time: NaiveTime) {
		if self.config.is_none() {
			self.config = Some(Config::default());
		}
		self.config = Some(Config {
			default_deadline: Some(time),
			..self.config.clone().unwrap()
		});
	}

	pub fn conf_get_default_deadline(&self) -> Option<NaiveTime> {
		self.config.as_ref().and_then(|c| c.default_deadline)
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
			config: None,
		}
	}
}
