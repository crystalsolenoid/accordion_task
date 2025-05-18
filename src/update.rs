use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::Input;

use crate::app::{App, Menu, Mode};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.menu_focus {
        Mode::Navigation => update_navigation_view(app, key_event),
        Mode::Typing(menu) => update_typing_view(app, key_event, menu),
    }
}

fn update_navigation_view(app: &mut App, key_event: KeyEvent) {
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
        KeyCode::Char('s') => app.attempt_skip(),
        KeyCode::Char('a') => app.append_task_start(),
        KeyCode::Char('i') => app.insert_task_start(),
        KeyCode::Char('?') => app.toggle_help(),
        KeyCode::Char('d') => app.toggle_debug(),
        _ => {}
    };
}

fn update_typing_view(app: &mut App, key_event: KeyEvent, menu: Menu) {
    match key_event.code {
        KeyCode::Esc => app.cancel_typing(),
        // submit
        KeyCode::Enter => app.submit_typing(menu),
        // TODO there are other ways to make a newline which I need to disable.
        _ => {
            app.text_input
                .input(<crossterm::event::KeyEvent as Into<Input>>::into(key_event));
        } //_ => {app.text_input.input(key_event.into());},
    }
}
