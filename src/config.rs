use color_eyre::eyre::{OptionExt, Result};
use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Config file format. This can be specified by the user by creating a `.toml` file in a
/// location that the [directories
/// crate's ProjectDirs config_local_dir method](https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.config_local_dir) can find.
///
/// To print the config path, run the CLI with the option `--config`.
#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub clock_format: ClockFormat,
}

/// Time display format
///
/// Used in places where space is confined in the UI. Times are implicitly close to the current
/// time (Accordion Task is not optimized for tasks long enough that this would be
/// ambiguous.) Thus, AM and PM are omitted.
#[derive(Deserialize, Default)]
pub enum ClockFormat {
    /// 24 hour format. Example: `15:00` or ` 1:15`
    #[default]
    #[serde(rename = "24hr")]
    Fmt24Hr,
    /// 12 hour format. Example: ` 3:00` or ` 1:00`
    /// AM and PM are not shown.
    #[serde(rename = "12hr")]
    Fmt12Hr,
    /// Custom format. Any valid [chrono
    /// strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) string is allowed.
    ///
    /// Warning: As of 2025-06-12, the UI is not responsive enough yet to handle
    /// displaying formats with an output wider than 5 characters.
    ///
    /// Example: `%H:%M` for zero-padded 24 hour times.
    Custom(String),
}

impl ClockFormat {
    pub fn get_strftime(&self) -> &str {
        match self {
            Self::Fmt24Hr => "%k:%M",
            Self::Fmt12Hr => "%l:%M",
            Self::Custom(s) => s,
        }
    }
}

/// # Errors
///
/// Will return an error if the home directory does not exist, or if it is inaccessible.
/// Does not check that the later parts of the path exist.
pub fn find_config_location() -> Result<PathBuf> {
    ProjectDirs::from("", "", "Accordion Task")
        .map(|dirs| {
            dirs.config_local_dir()
                .to_owned()
                .join("config.toml")
        })
        .ok_or_eyre("Could not find a config path.")
}

fn try_load() -> Result<Config> {
    let dir = find_config_location()?;
    let conf_str = fs::read_to_string(dir)?;
    let config = toml::from_str(&conf_str)?;
    Ok(config)
}

/// Tries to load and parse the config file. If that fails, returns the default config.
pub fn load() -> Config {
    match try_load() {
        Err(e) => {
            println!("Warning: Config file failed with error {e} Falling back to default config.");
            Config::default()
        }
        Ok(c) => c,
    }
}
