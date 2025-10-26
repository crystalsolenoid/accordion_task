// TODO track time with chrono
// TODO wrap a Routine in a Session
// which has state for the current session
// like the active task and the start/end times

use accordion_core::routine::template::{
    RoutineTemplate, RoutineTemplateStoreFields, TaskTemplate, TaskTemplateStoreFields,
};
use chrono::{DateTime, Local, NaiveTime};
use std::time::Duration;

use gloo_file::Blob as GlooBlob;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::{Blob, File, HtmlInputElement};
use leptos_router::{
    components::{A, Form, Route, Router, Routes},
    path,
};
use reactive_stores::{Field, Store, StoreFieldIterator};

use accordion_core::routine::{
    self, CompletionStatus, Routine, RoutineStoreFields, Task, task::TaskStoreFields,
};
use accordion_core::session::{Session, SessionStoreFields};
use accordion_core::utils;

use leptos::Params;
use leptos_router::hooks::{use_params, use_query};
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};

// TODO rename
#[derive(Params, PartialEq)]
struct ContactSearch {
    d: Option<NaiveTime>,
}

// TODO rename
#[derive(Params, PartialEq)]
struct ContactParams {
    name: Option<String>,
}

#[component]
fn Duration(#[prop(into)] value: Signal<Duration>) -> impl IntoView {
    view! {
        {{ move || utils::format_duration(value.get()) }}
    }
}

#[component]
fn RoutineTimer(#[prop(into)] session: Field<Session>) -> impl IntoView {
    let elapsed = Memo::new(move |_| session.tasks().read().elapsed());
    let remaining = Memo::new(move |_| session.tasks().read().remaining());
    let eta = move || {
        session
            .read()
            .get_projected_end_time()
            .format(TIME_FORMAT)
            .to_string()
    };
    view! {
        <p>Remaining: <Duration value=remaining /></p>
        <p>Elapsed: <Duration value=elapsed /></p>
        <p>Expected End: {{ move || eta() }}</p>
    }
}

#[component]
fn TaskListItem(#[prop(into)] task: Field<Task>) -> impl IntoView {
    view! {
        <h2>{{ move || task.name().get() }}</h2>
        <span>
            "("
            {{
                move || match task.status().get() {
                    CompletionStatus::NotYet => "Incomplete",
                    CompletionStatus::Done => "Done",
                    CompletionStatus::Skipped => "Skipped",
                }
            }} ")"
        </span>
        <div>
            <span>
                <Duration value=task.elapsed() />
            </span>
            <progress value=move || {
                task.elapsed().get().as_secs() as f64 / task.duration().get().as_secs() as f64
            } />
            <span>
                <Duration value=task.duration() />
            </span>
        </div>
    }
}

#[component]
fn RoutinePlayer(#[prop(into)] session: Field<Session>, initial_active: usize) -> impl IntoView {
    view! {
        <RoutineTimer session=session />
        <div id="task-actions">
            <button on:click=move |_| {
                let _ = session.write().toggle();
            }>Complete Current</button>
            <button on:click=move |_| {
                let _ = session.write().skip();
            }>Skip Current</button>
        </div>
        <ol class="routine">
            <For
                each=move || session.tasks().tasks().into_iter().enumerate()
                key=|(_, task)| task.read().name.clone()
                children=move |(i, child)| {
                    view! {
                        <li class="task">
                            <label for=child.clone().name()>
                                <TaskListItem task=child.clone() />
                            </label>
                            <input
                                type="radio"
                                class="active-task"
                                value=i
                                id=child.name()
                                name="active"
                                prop:checked=i == initial_active
                                on:change=move |_| {
                                    session.selected().write().select(Some(i));
                                    SessionStorage::set("in-progress-session", session.get());
                                }
                            />
                        </li>
                    }
                }
            />
        </ol>
    }
}

#[component]
fn SavedRoutineViewer(#[prop(into)] data: Field<Session>) -> impl IntoView {
    let params = use_params::<ContactParams>();
    let name = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.name.clone()) // TODO lazy clone
            .unwrap_or_default()
    };

    let routines: StoredRoutines = LocalStorage::get("stored-routines").unwrap_or_default();
    let routines_store: Store<StoredRoutines> = Store::new(routines);

    Effect::new(move |_| {
        LocalStorage::set("stored-routines", routines_store.get());
    });

    let routine_store = move || {
        routines_store
            .vec_field()
            .iter_unkeyed()
            .find(|rs| rs.name().get() == name())
            .unwrap()
    };

    view! {
        <h1>
            {{move || routine_store().name()}}
        </h1>
        // TODO instead, link to a page
        // thats for that routine? Maybe?
        <button on:click=move |_| {
            let new_session = Session::new(routine_store().get());
            data.set(new_session);
            SessionStorage::set("in-progress-session", data.get());
        }>Overwrite Active Routine</button>
        <PreviewRoutine routine=routine_store() />
    }
}

#[component]
fn PreviewRoutine(#[prop(into)] routine: Field<RoutineTemplate>) -> impl IntoView {
    let (edit, set_edit) = signal(None::<usize>);
    view! {
        <table>
        <thead>
            <tr>
                <th>Name</th>
                <th>Starting Duration</th>
            </tr>
        </thead>
        <tbody>
        <ForEnumerate
            each=move || routine.tasks().iter_unkeyed()
            key=|task| task.id().get()
            children=move |_i, task| {
                view!{
                    <tr>
                        <td>
                        {task.name().get()}
                        </td>
                        <td>
                        <Duration value=task.duration().with(|d| Duration::from_secs(*d)) />
                        </td>
                    </tr>
                }
            }
        />
        </tbody>
        </table>
    }
}

#[component]
fn DeadlinePicker() -> impl IntoView {
    view! {
        <Form method="GET" action="">
            <label>
            {{ "Set Deadline" }}
            <input type="time" name="d" />
            </label>
            <button>Submit</button>
        </Form>
        {{ move || use_query::<ContactSearch>()
            .read()
            .as_ref()
            .ok()
            .and_then(|queries| queries.d)
            .map(|d| d.format(TIME_FORMAT).to_string())
             }}
    }
}

#[derive(Default, Store, Deserialize, Serialize, Clone, PartialEq, Eq)]
struct StoredRoutines {
    vec_field: Vec<RoutineTemplate>,
}

#[component]
fn Picker() -> impl IntoView {
    let routines: StoredRoutines = LocalStorage::get("stored-routines").unwrap_or_default();
    let routines_store: Store<StoredRoutines> = Store::new(routines);

    Effect::new(move |_| {
        LocalStorage::set("stored-routines", routines_store.get());
    });

    let (last_delete, set_last_delete) = signal(None::<RoutineTemplate>);

    view! {
        "Pick a routine."
        <ul>
        {move || routines_store.vec_field().iter_unkeyed().enumerate()
            .map(|(i, routine)| view!{
                <li>
                <A href=move || "routine/".to_string()+&routine.name().get()>{{routine.name()}}</A>
                <button on:click = move |_| {
                    let new_routine = routine.get();
                    let new_name = routine.name().get() + "cloned";
                    routines_store.vec_field().write().push(RoutineTemplate::new(new_name, vec![]));
                }>clone</button>
                <button on:click = move |_| {
                    set_last_delete.set(Some(routines_store.vec_field().write().remove(i)));
                }>delete</button>
                </li>
            })
            .collect_view()
        }
        </ul>
        <Show
            when=move || {last_delete.get().is_some()}
            fallback=|| view! {}
        >
            <button on:click=move |_| {
                routines_store.vec_field().write().push(last_delete.get().unwrap());
                set_last_delete.set(None);
            }>Restore last deleted: {move || last_delete.get().unwrap().name}</button>
        </Show>
    }
}

const TIME_FORMAT: &str = "%-I:%M %p";

// TODO I need a better way to set the current routine
#[component]
fn Upload(#[prop(into)] data: Field<Session>) -> impl IntoView {
    let preview_routine = Store::new(RoutineTemplate::default());
    let (name, set_name) = signal("New Routine".to_string());
    view! {
        // TODO make this a form
        <h1>Upload Routine</h1>
        <input
            type="file"
            accept=".routine"
            on:change=move |ev| {
                let target = ev.target().unwrap();
                let input = target.dyn_ref::<HtmlInputElement>().unwrap();
                let blob: GlooBlob = input.files().and_then(|files| files.item(0)).unwrap().into();
                spawn_local(async move {
                    let contents = gloo_file::futures::read_as_text(&blob).await;
                    let mut routine = routine::parse::from_csv(contents.unwrap().as_bytes()).unwrap();
                    routine.name = name.get();
                    preview_routine.set(routine);
                });
            }
        />
        <Show when=move || { preview_routine.tasks().read().len() != 0 }>
            <button on:click=move |_| {
                let new_session = Session::new(preview_routine.get());
                data.set(new_session);
                SessionStorage::set("in-progress-session", data.get());
            }>Overwrite Active Routine</button>

            <label>Routine Name
                <input id="routine-name"
                    on:input:target=move |ev| {
                        let routine_name = ev.target().value();
                        set_name.set(routine_name.clone());
                        preview_routine.name().set(routine_name);
                    }
                    prop:value=name
                />
            </label>

            <button on:click=move |_| {
                let mut stored_routines: StoredRoutines =
                    LocalStorage::get("stored-routines")
                    .unwrap_or_default();
                let already_exists = stored_routines.vec_field.iter()
                    .find(|r| r.name == name.get())
                    .is_some();
                if !already_exists {
                    stored_routines.vec_field.push(preview_routine.get());
                    LocalStorage::set("stored-routines", stored_routines);
                }
            }>Save Routine</button>

            <h2>
                {{move || preview_routine.name().get()}} <em> (Preview)</em>
            </h2>
            <PreviewRoutine routine=preview_routine />
        </Show>
    }
}

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
        let deadline = use_query::<ContactSearch>()
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
