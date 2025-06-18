use csv::{StringRecord, Trim};
use std::{env, error::Error, ffi::OsString, fs::File};

use super::Task;

// TODO what's a better way to specify this path?
use crate::routine::task::parse_new::parse_duration;

fn run() -> Result<Vec<Task>, Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .trim(Trim::All)
        .comment(Some(b'#'))
        .from_reader(file);
    let mut tasks = Vec::<Task>::new();
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        tasks.push(parse_task(&record));
    }
    Ok(tasks)
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

pub fn read_csv() -> Result<Vec<Task>, Box<dyn Error>> {
    run()
}

fn parse_task(record: &StringRecord) -> Task {
    Task::new(
        record.get(0).expect("Missing CSV field."),
        parse_duration(record.get(1).expect("Missing CSV field."))
            .expect("Failure parsing duration. Format: _h_m_s"),
    )
}
