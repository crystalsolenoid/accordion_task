use chrono::NaiveTime;
use clap::Parser;

#[derive(Parser)]
#[command(
    arg_required_else_help = true,
    version = "alpha",
    about = "personal routine timing assistant"
)]
pub struct Cli {
    /// Routine path
    #[arg()]
    pub routine_path: String,
    /// Deadline
    #[arg(short)]
    pub deadline: Option<NaiveTime>,
}
