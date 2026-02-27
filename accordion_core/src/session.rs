pub mod selection;
pub use selection::ListPointer;

use std::time::Duration;

use chrono::{DateTime, Local, TimeDelta, Utc};
use color_eyre::Result;

use crate::routine::{CompletionStatus, Routine, ToggleFailure, template::RoutineTemplate};

#[cfg(feature = "web")]
use reactive_stores::Store;
#[cfg(feature = "web")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "web", derive(Store, Clone, Serialize, Deserialize))]
pub struct Session {
	pub start_time: DateTime<Local>,
	pub last_tick: DateTime<Utc>,
	// pub deadline: DateTime<Local>,
	pub tasks: Routine,
	pub selected: ListPointer,
}

impl Session {
	pub fn new(template: RoutineTemplate) -> Self {
		let tasks = template.generate_routine();
		let length = tasks.tasks.len();
		Self {
			start_time: Local::now(),
			tasks,
			last_tick: Utc::now(),
			selected: ListPointer::new(length),
		}
	}
}

impl Session {
	/// Handles the tick event of the terminal.
	pub fn tick(&mut self) -> TimeDelta {
		let this_tick = Utc::now();
		let delta = this_tick - self.last_tick;
		self.last_tick = this_tick;

		self.tasks
			.elapse(self.selected.selected(), delta.to_std().unwrap());
		delta
	}

	pub fn get_percentage_elapsed(&self) -> f64 {
		// TODO should the percentage bar be configurable? Another option could be
		// number of tasks completed, which would make the effort of task
		// switching more recognized. The value of leaving it this way even then is honing
		// your time understanding skills.
		self.tasks
			.completed_originals()
			.div_duration_f64(self.tasks.total_originals())
	}

	pub fn get_total_remaining(&self) -> Duration {
		self.tasks.remaining()
	}

	pub fn get_projected_end_time(&self) -> DateTime<Local> {
		Local::now() + self.get_total_remaining()
	}

	pub fn toggle(&mut self) -> Result<CompletionStatus, ToggleFailure> {
		let i = self.selected.selected();
		self.tasks.toggle(i)
	}

	pub fn skip(&mut self) -> Result<CompletionStatus, ToggleFailure> {
		let i = self.selected.selected();
		self.tasks.skip(i)
	}

	pub fn set_deadline(&mut self, deadline: DateTime<Local>) {
		self.tasks.set_deadline(deadline);
	}
}
