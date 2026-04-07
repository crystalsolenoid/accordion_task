use std::{fmt::Display, time::Duration};

#[cfg(feature = "web")]
use reactive_stores::Store;
#[cfg(feature = "web")]
use serde::{Deserialize, Serialize};

pub mod parse_new;
pub use parse_new::parse_new;

use crate::utils;

#[cfg_attr(feature = "web", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub enum CompletionStatus {
	NotYet,
	Done,
	Skipped,
}

#[cfg_attr(feature = "web", derive(Serialize, Deserialize, Eq))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FlexDuration {
	/// Start at T and either contract or stay at a maximum T.
	Shrinking(Duration),
	/// Start at T and either contract or expand unbounded.
	Growing(Duration),
}

impl Display for FlexDuration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", utils::format_flex_duration(self))
	}
}

impl FlexDuration {
	pub fn init_duration(&self) -> Duration {
		match self {
			FlexDuration::Shrinking(d) => *d,
			FlexDuration::Growing(d) => *d,
		}
	}
}

#[derive(Debug)]
#[cfg_attr(feature = "web", derive(Store, Clone, Serialize, Deserialize))]
pub struct Task {
	/// How much time has already been spent on the task?
	pub elapsed: Duration,
	/// What was the original duration specified for the task?
	pub duration_spec: FlexDuration,
	/// Is the task completed?
	pub status: CompletionStatus,
	/// Name
	pub name: String,
	/// Current duration that may be shrunk
	pub current_duration: Duration,
	pub id: usize,
}

impl Task {
	pub fn new(name: &str, duration: FlexDuration, id: usize) -> Self {
		Self {
			name: name.to_owned(),
			elapsed: Duration::ZERO,
			duration_spec: duration,
			current_duration: duration.init_duration(),
			status: CompletionStatus::NotYet,
			id,
		}
	}

	pub fn init_duration(&self) -> Duration {
		self.duration_spec.init_duration()
	}

	pub fn remaining(&self) -> Duration {
		self.current_duration.saturating_sub(self.elapsed)
	}

	pub fn elapse(&mut self, duration: Duration) {
		self.elapsed += duration;
	}
}
