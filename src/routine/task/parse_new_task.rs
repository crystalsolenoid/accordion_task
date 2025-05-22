// This module is for parsing tasks that are created on the fly.
// Free form parsing, like Remember the Milk but simpler
// examples:
// shower 15m
// should create a new task that is named "shower" and has an initial duration of
// 15mins.
//
// ambiguity: should tasks created with a specified time have that as their original
// time, or should they compress as necessary for the current time budget?
use std::str::FromStr;
use std::num::ParseIntError;

// TODO consider refactoring to use Winnow? Keep an eye out for if/when it isn't overkill

/// Returns the number of seconds.
pub fn parse_duration(raw: &str) -> Result<u64, ParseIntError> {
    // TODO this code is so old and weird. I surely know how to do it more idiomatically now.
    let mut number_accum = String::new();
    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;
    for g in raw.chars() {
        match g {
            'h' => {
                hours = u64::from_str(&number_accum)?;
                number_accum = String::new();
            }
            'm' => {
                minutes = u64::from_str(&number_accum)?;
                number_accum = String::new();
            }
            's' => {
                seconds = u64::from_str(&number_accum)?;
                number_accum = String::new();
            }
            ' ' => (),
            _ => number_accum.push(g),
        }
    }
    Ok(hours * 60 * 60 + minutes * 60 + seconds)
}
