use csv::{StringRecord, Trim};
use std::{env, error::Error, ffi::OsString, fs::File, process};

use super::Task;

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
        println!("{:?}", record);
        tasks.push(parse_task(record));
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

pub fn parse_task(record: StringRecord) -> Task {
    Task {
        title: record.get(0).expect("Missing CSV field.").to_string(),
        ..Task::from_secs(20)
    }
}
