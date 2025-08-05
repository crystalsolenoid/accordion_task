use color_eyre::{
    eyre::{OptionExt, WrapErr},
    Result,
};
use csv::{StringRecord, Trim};

use std::io;

use super::{Task, task::parse_new::parse_duration};

pub fn from_csv(r: impl io::Read) -> Result<Vec<Task>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .trim(Trim::All)
        .comment(Some(b'#'))
        .from_reader(r);
    let mut tasks = Vec::<Task>::new();
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        tasks.push(parse_task(&record)?);
    }
    Ok(tasks)
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
