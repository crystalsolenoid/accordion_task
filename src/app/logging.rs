// Log actions the user takes
// which are saved as a timestamped file
// to be used for future insights in routine
// planning.
//
// prototype
//
// time | task | action
//
// example actions:
// time elapsed
// task completed
// task uncompleted
//
// 7:36pm   brush teeth 2m30s
//
// cull entries shorter than a certain threshold
// if the task wasn't marked completed,
// because it is probably from the user skipping
// past the task to focus on another
//
// have headers with global context
// including the date the routine was initiated on
// this keeps the lines more human-readable
//
// routine name | date-time started | duration
//
// include on-the-fly routine refinement notes
// or maybe allow a way to add a task on the fly
// and mark it as a task you want added into the
// routine for real

use chrono::{DateTime, Local};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Duration;

use crate::app::{Routine, Task};

#[derive(Debug, Copy, Clone)]
enum LogEvent {
    Elapsed(Duration),
    Complete(bool),
    Skip(bool),
}

/*
* TODO make a trait Loggable
* and implement it for LogElement (renamed TaskLog) and Note
*/

#[derive(Debug)]
pub struct LogElement {
    time: DateTime<Local>,
    task_name: String,
    event: LogEvent,
}

impl LogElement {
    fn new(task: &Task, event: LogEvent) -> LogElement {
        LogElement {
            time: Local::now(),
            task_name: task.name.clone(), //TODO dont clone? unsure
            event,
        }
    }

    pub fn combine(self, next: Self) -> (Self, Option<Self>) {
        // only combine two elapse events,
        // and only if they are for the same
        // task
        if self.task_name == next.task_name {
            match (self.event, next.event) {
                (LogEvent::Elapsed(a), LogEvent::Elapsed(b)) => (
                    Self {
                        event: LogEvent::Elapsed(a + b),
                        ..self
                    },
                    None,
                ),
                (_, _) => (self, Some(next)),
            }
        } else {
            (self, Some(next))
        }
    }

    pub fn write(&self, file: &mut BufWriter<File>) {
        let time = self.time.format("%T");
        let name = &self.task_name;
        let message = match self.event {
            LogEvent::Elapsed(d) => {
                format!("{} elapsed", crate::ui::format_duration(d))
            }
            LogEvent::Complete(true) => "completed".to_string(),
            LogEvent::Complete(false) => "uncompleted".to_string(),
            LogEvent::Skip(true) => "skipped".to_string(),
            LogEvent::Skip(false) => "unskipped".to_string(),
        };
        //        let filename = format!("{}-{}", routine_name, start_time.format("%FT%T"));

        writeln!(file, "{time} \t{name} \t{message:}").unwrap();

        // 7:36pm   brush teeth 2m30s
    }

    pub fn elapsed(task: &Task, elapsed: Duration) -> LogElement {
        Self::new(task, LogEvent::Elapsed(elapsed))
    }

    pub fn completed(task: &Task) -> LogElement {
        Self::new(task, LogEvent::Complete(true))
    }

    pub fn uncompleted(task: &Task) -> LogElement {
        Self::new(task, LogEvent::Complete(false))
    }

    pub fn skipped(task: &Task) -> LogElement {
        Self::new(task, LogEvent::Skip(true))
    }

    pub fn unskipped(task: &Task) -> LogElement {
        Self::new(task, LogEvent::Skip(false))
    }
}

trait Loggable {
    fn contents(&self) -> String;
}

#[derive(Debug)]
pub struct RoutineLogger {
    file: BufWriter<File>,
    event_buffer: Vec<LogElement>,
}

impl RoutineLogger {
    pub fn new(
        _routine: &Routine,
        start_time: &DateTime<Local>,
        routine_name: String,
    ) -> RoutineLogger {
        let filename = format!("{}-{}", routine_name, start_time.format("%FT%T"));
        let file = File::create(filename).expect("failed to create file");
        let file = BufWriter::new(file);

        RoutineLogger {
            file,
            event_buffer: vec![],
        }
    }

    pub fn log(&mut self, event: LogElement) {
        if let Some(e) = self.event_buffer.pop() {
            let (a, b) = e.combine(event);
            if let Some(e) = b {
                self.write(a);
                self.event_buffer.push(e);
            } else {
                self.event_buffer.push(a);
            }
        } else {
            self.event_buffer.push(event);
        }
    }

    fn write(&mut self, log: LogElement) {
        log.write(&mut self.file);
    }

    pub fn finish(&mut self) {
        if let Some(e) = self.event_buffer.pop() {
            self.write(e);
        }
    }
}

mod test {
    #[test]
    fn api() {
        let task = Task::new("test", 120);
        let time = DateTime::from_timestamp(1431648000, 0)
            .expect("invalid timestamp");
        let logger = Logger::new(&routine, time);

        let log = completed(task, time);
        logger.push(log);
    }

    #[test]
    fn pause() {
        let task = Task::new("test", 120);
        let time = DateTime::from_timestamp(1431648000, 0)
            .expect("invalid timestamp");
        let logger = Logger::new(&routine, time);

        let note = "reason for interruption";

        let log = paused(note, time);
        logger.push(log);
    }
}
