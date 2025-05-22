use std::time::Duration;

pub mod parse_new_task;

#[derive(Debug, Copy, Clone)]
pub enum CompletionStatus {
    NotYet,
    Done,
    Skipped,
}

#[derive(Debug)]
pub struct Task {
    /// How much time has already been spent on the task?
    pub elapsed: Duration,
    /// What was the original duration specified for the task?
    pub original_duration: Duration,
    /// Is the task completed?
    pub status: CompletionStatus,
    /// Name
    pub name: String,
    /// Current duration that may be shrunk
    pub duration: Duration,
}

impl Task {
    pub fn new(name: &str, duration: u64) -> Self {
        Self {
            name: name.to_owned(),
            elapsed: Duration::ZERO,
            original_duration: Duration::new(duration, 0),
            duration: Duration::new(duration, 0),
            status: CompletionStatus::NotYet,
        }
    }

    pub fn remaining(&self) -> Duration {
        self.duration.saturating_sub(self.elapsed)
    }

    pub fn elapse(&mut self, duration: Duration) {
        self.elapsed += duration;
    }
}
