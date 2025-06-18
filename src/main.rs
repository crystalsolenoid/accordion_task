/// Application.
pub mod app;

/// Routine.
pub mod routine;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

/// Command line interface parser.
pub mod cli;

/// Config file
pub mod config;

use app::App;
use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    cli_log::init_cli_log!();

    let cli = Cli::parse();

    if cli.run_instead_of_tui() {
        return Ok(());
    }

    // Create an application.
    let mut app = App::new(cli);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(500);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        // Explanation for allow: I intend for Mouse to stay empty, but intend
        // for Resize to get filled in later.
        #[allow(clippy::match_same_arms)]
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {
                // I do not plan on adding mouse functionality for a TUI
                // unless there are strong requests for it, because I
                // don't like it.
            }
            Event::Resize(_, _) => {
                //todo!("must handle resizing before release!")
            }
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
