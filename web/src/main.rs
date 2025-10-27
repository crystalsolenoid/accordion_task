// TODO track time with chrono
// TODO wrap a Routine in a Session
// which has state for the current session
// like the active task and the start/end times

use accordion_core::routine::template::{RoutineTemplate, TaskTemplate};
use chrono::Local;
use std::time::Duration;
use web::components::{DeadlinePicker, Picker, RoutinePlayer, SavedRoutineViewer, Upload};
use web::router::DeadlineQuery;

use gloo_storage::{SessionStorage, Storage};
use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};
use reactive_stores::Store;

use accordion_core::session::Session;

use leptos_router::hooks::use_query;

#[component]
fn App() -> impl IntoView {
    let initial_active: usize;
    let session = if let Ok(session) = SessionStorage::get("in-progress-session") {
        let session: Session = session;
        initial_active = session.selected.selected().unwrap_or_default();
        Store::new(session)
    } else {
        let mut list = RoutineTemplate::default();
        list.push(TaskTemplate::new("shower", 120, 0));
        list.push(TaskTemplate::new("eat dinner", 60, 1));
        list.push(TaskTemplate::new("program", 9990, 2));
        let session = Session::new(list);
        initial_active = session.selected.selected().unwrap_or_default();
        Store::new(session)
    };

    Effect::new(move || {
        // Is an effect really the best way to do this?
        // Is it a reasonable use of an Effect?
        // TODO Find out what the other options are...
        let deadline = use_query::<DeadlineQuery>()
            .read()
            .as_ref()
            .ok()
            .and_then(|queries| queries.d)
            .map(|d| accordion_core::utils::interpret_naive_time(Local::now(), d));
        if let Some(d) = deadline {
            session.write().set_deadline(d);
        }
    });

    leptos::leptos_dom::helpers::set_interval(
        move || {
            session.write().tick();
        },
        // BUG: if this is a larger number ( try 10 seconds ), the
        // duration-related reactive components... won't react??!
        Duration::from_millis(50),
    );

    leptos::leptos_dom::helpers::set_interval(
        move || {
            SessionStorage::set("in-progress-session", session.get());
        },
        Duration::from_secs(60),
    );

    view! {
        <Router>
            <nav>
                <A href="">Picker</A>
                <A href="routine">Routine</A>
                <A href="upload">Upload</A>
            </nav>
            <Routes fallback=|| "">
                <Route path=path!("") view=Picker/>
                <Route path=path!("/upload") view=move || view! { <Upload data=session /> } />
                <Route
                    path=path!("/routine")
                    view=move || {
                        view! {
                            <DeadlinePicker/>
                            <RoutinePlayer session=session initial_active=initial_active />
                        }
                    }
                />
                <Route path=path!("/routine/:name") view=move || {view!{<SavedRoutineViewer data=session/>}}/>
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
