use color_eyre::{
    eyre::{OptionExt, WrapErr},
    Result,
};
use csv::{StringRecord, Trim};
use std::{env, ffi::OsString, fs::File};

use super::Task;

// TODO what's a better way to specify this path?
use crate::routine::task::parse_new::parse_duration;

fn run() -> Result<Vec<Task>> {
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
        tasks.push(parse_task(&record)?);
    }
    Ok(tasks)
}

fn get_first_arg() -> Result<OsString> {
    // TODO should i use CLAP instead here
    env::args_os()
        .nth(1)
        .ok_or_eyre("Expected 1 argument, got none.")
}

pub fn read_csv() -> Result<Vec<Task>> {
    run()
}

fn parse_task(record: &StringRecord) -> Result<Task> {
    Ok(Task::new(
        record
            .get(0)
            .ok_or_eyre("Missing CSV field. Note: Blank lines not yet allowed.")?,
        parse_duration(record.get(1).ok_or_eyre("Missing CSV field.")?)
            .wrap_err("Failure parsing duration. Format: _h_m_s")?,
    ))
}
