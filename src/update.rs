use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Right | KeyCode::Char('j') => app.next_task(),
        KeyCode::Left | KeyCode::Char('k') => app.prev_task(),
        KeyCode::Enter => app.attempt_toggle(),
        _ => {}
    };
}
