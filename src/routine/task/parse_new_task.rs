// This module is for parsing tasks that are created on the fly.
// Free form parsing, like Remember the Milk but simpler
// examples:
// shower 15m
// should create a new task that is named "shower" and has an initial duration of
// 15mins.
//
// ambiguity: should tasks created with a specified time have that as their original
// time, or should they compress as necessary for the current time budget?
use std::num::ParseIntError;
use std::str::FromStr;

use crate::routine::Task;

const DEFAULT_DURATION_SECS: u64 = 5 * 60;

// TODO consider refactoring to use Winnow? Keep an eye out for if/when it isn't overkill

pub fn parse_new_task(raw: &str) -> Task {
    // TODO figure out error type. I want it to fail silently most of the time if
    // duration parsing fails, but if the task is nameless and durationless, assume it
    // was a mistake and don't create the new empty task.
    // - find the final delimiter (whitespace) not counting trailing whitespace
    // - split on that character
    // - attempt to parse the latter chunk as a duration
    // -- if it works, set the name as the former chunk and the duration as the
    // parsing result
    // -- if it fails, assume the full unsplit chunk is the name and set the duration to
    // the default.
    // - create and return the task
    let default_duration = DEFAULT_DURATION_SECS;
    let (name, duration) = match raw.rsplit_once(' ') {
        None => (raw.to_owned(), default_duration),
        Some((name, possible_duration)) => {
            let name = name.to_owned();
            match parse_duration(possible_duration) {
                Ok(secs) => (name, secs),
                Err(_) => (name + " " + possible_duration, default_duration),
            }
            // TODO remove unwrap
            //todo!("{}", duration.as_secs());
        }
    };
    Task::new(&name, duration)
}

/// Returns the number of seconds.
pub fn parse_duration(raw: &str) -> Result<u64, ParseIntError> {
    // TODO this code is so old and weird. I surely know how to do it more idiomatically now.
    let mut number_accum = String::new();
    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;
    let mut seen_hms = false;
    for g in raw.chars() {
        match g {
            'h' => {
                hours = u64::from_str(&number_accum)?;
                number_accum = String::new();
                seen_hms = true;
            }
            'm' => {
                minutes = u64::from_str(&number_accum)?;
                number_accum = String::new();
                seen_hms = true;
            }
            's' => {
                seconds = u64::from_str(&number_accum)?;
                number_accum = String::new();
                seen_hms = true;
            }
            ' ' => (),
            _ => number_accum.push(g),
        }
    }
    match seen_hms {
        true => Ok(hours * 60 * 60 + minutes * 60 + seconds),
        // TODO this is an ugly, ugly hack because I don't want to come up with
        // a new result type right now.
        false => Ok(u64::from_str("")?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    #[test]
    fn parse_task_with_duration() {
        let input = "wash clothes 5m30s";

        let task = parse_new_task(input);

        assert_eq!(task.name, "wash clothes");
        assert_eq!(task.original_duration, Duration::from_secs(5 * 60 + 30));
    }

    #[test]
    fn no_duration() {
        let input = "shower";

        let task = parse_new_task(input);

        assert_eq!(task.name, "shower");
    }

    #[test]
    fn default_time() {
        let input = "shower";

        let task = parse_new_task(input);

        assert_eq!(task.original_duration, Duration::from_secs(5 * 60));
        // TODO: how will i decide a default?
    }

    #[test]
    fn no_duration_with_space() {
        let input = "wash clothes";

        let task = parse_new_task(input);

        assert_eq!(task.name, "wash clothes");
    }

    #[test]
    fn no_hms_task() {
        let input = "dishes away";

        let task = parse_new_task(input);

        assert_eq!(task.name, "dishes away");
    }

    #[test]
    fn no_hms_duration() {
        assert!(parse_duration("away").is_err());
    }
}
