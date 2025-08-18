use color_eyre::{
    Result,
    eyre::{OptionExt, WrapErr},
};
use csv::{StringRecord, Trim};

use std::io;

use super::{
    Task,
    task::parse_new::parse_duration,
    template::{RoutineTemplate, TaskTemplate},
};

pub fn from_csv(r: impl io::Read) -> Result<RoutineTemplate> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .trim(Trim::All)
        .comment(Some(b'#'))
        .from_reader(r);
    let mut tasks = Vec::<TaskTemplate>::new();
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        tasks.push(parse_task(&record)?);
    }
    Ok(RoutineTemplate {
        name: "Current Routine".to_string(),
        tasks,
    })
}

fn parse_task(record: &StringRecord) -> Result<TaskTemplate> {
    Ok(TaskTemplate {
        name: record
            .get(0)
            .ok_or_eyre("Missing CSV field. Note: Blank lines not yet allowed.")?
            .to_string(),
        duration: parse_duration(record.get(1).ok_or_eyre("Missing CSV field.")?)
            .wrap_err("Failure parsing duration. Format: _h_m_s")?,
    })
}
