use chrono::NaiveTime;
use clap::Parser;

use crate::config;

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
