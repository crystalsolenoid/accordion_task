use color_eyre::{
    Result,
    eyre::{OptionExt, WrapErr},
};
use csv::{StringRecord, Trim};

use std::io;

use super::{
    Task,
    task::parse_new::parse_duration,
    template::{self, RoutineTemplate, TaskTemplate},
};

pub fn from_csv(mut r: impl io::Read) -> Result<RoutineTemplate> {
    // Put everything into a string because
    // it's so much simpler dammit.
    let mut combined = String::new();
    r.read_to_string(&mut combined)?;

    // First check for a config header
    let mut as_lines = combined.lines();
    let has_config = as_lines.next().unwrap() == "---";
    let raw_config: String = if has_config {
        as_lines
            .by_ref()
            .take_while(|l| *l != "---")
            .map(|l| l.to_owned() + "\n")
            .collect()
    } else {
        String::new()
    };
    let tasks_csv: String = if has_config {
        as_lines.map(|l| l.to_owned() + "\n").collect()
    } else {
        combined
    };
    let config: template::Config = toml::from_str(&raw_config).unwrap();

    // Then get the tasks
    let mut counter = 0;
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .trim(Trim::All)
        .comment(Some(b'#'))
        .from_reader(tasks_csv.as_bytes());
    let mut tasks = Vec::<TaskTemplate>::new();
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        tasks.push(parse_task(&record, counter)?);
        counter += 1;
    }
    Ok(RoutineTemplate::with_config(
        "Current Routine".to_string(),
        tasks,
        config,
    ))
}

fn parse_task(record: &StringRecord, id: usize) -> Result<TaskTemplate> {
    Ok(TaskTemplate {
        name: record
            .get(0)
            .ok_or_eyre("Missing CSV field. Note: Blank lines not yet allowed.")?
            .to_string(),
        duration: parse_duration(record.get(1).ok_or_eyre("Missing CSV field.")?)
            .wrap_err("Failure parsing duration. Format: _h_m_s")?,
        id,
    })
}
