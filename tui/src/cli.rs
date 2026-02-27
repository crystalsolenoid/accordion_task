use chrono::NaiveTime;
use clap::Parser;
use color_eyre::{Result, eyre::OptionExt};

use std::{env, ffi::OsString, fs::File};

use accordion_core::routine::{self, template::RoutineTemplate};

use crate::config;

pub fn get_routine() -> Result<RoutineTemplate> {
	let file_path = get_first_arg()?;
	let file = File::open(file_path)?;
	routine::parse::from_csv(file)
}

fn get_first_arg() -> Result<OsString> {
	// TODO should i use CLAP instead here
	env::args_os()
		.nth(1)
		.ok_or_eyre("Expected 1 argument, got none.")
}

#[derive(Parser)]
#[command(
	arg_required_else_help = true,
	version = "alpha",
	about = "personal routine timing assistant"
)]
pub struct Cli {
	/// Routine path
	#[arg()]
	pub routine_path: Option<String>,
	/// Deadline
	#[arg(short)]
	pub deadline: Option<NaiveTime>,
	/// Print config search path
	#[arg(long = "config")]
	pub config_path: bool,
}

impl Cli {
	pub fn run_instead_of_tui(&self) -> bool {
		if self.config_path {
			match config::find_config_location() {
				Ok(p) => println!("{}", p.display()),
				Err(_) => println!("Could not access home directory."),
			}
			true
		} else {
			false
		}
	}
}
