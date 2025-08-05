// TODO track time with chrono
// TODO wrap a Routine in a Session
// which has state for the current session
// like the active task and the start/end times

use leptos::prelude::*;
use leptos_router::{path, components::{A, Router, Route, Routes}};
use leptos::task::spawn_local;
use std::time::Duration;

use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::{File, Blob, HtmlInputElement};

use gloo_storage::{Storage, SessionStorage};
use gloo_file::Blob as GlooBlob;

use reactive_stores::{Store, Field};
use accordion_core::routine::RoutineStoreFields;
use accordion_core::routine::task::TaskStoreFields;

use accordion_core::routine::{self, Routine, Task, CompletionStatus};
use accordion_core::utils;

#[component]
fn Duration(
    #[prop(into)]
    value: Signal<Duration>) -> impl IntoView {
    view! { {{ move || utils::format_duration(value.get()) }} }
}

#[component]
fn RoutineTimer(
    #[prop(into)]
    routine: Field<Routine>,
) -> impl IntoView {
    let elapsed = Memo::new(move |_| routine.get().elapsed());
    let remaining = Memo::new(move |_| routine.get().remaining());
    view! {
        <p>Remaining: <Duration value=remaining /></p>
        <p>Elapsed: <Duration value=elapsed /></p>
    }
}

#[component]
fn TaskListItem(
    #[prop(into)]
    task: Field<Task>,
) -> impl IntoView {
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
fn RoutinePlayer(
    #[prop(into)]
    routine: Field<Routine>,
    active: ReadSignal<Option<usize>>,
    set_active: WriteSignal<Option<usize>>,
    initial_active: usize
) -> impl IntoView {

    view! {
        <RoutineTimer routine=routine />
        <button on:click=move |_| {
            let _ = routine.write().toggle(active.get());
        }>Complete Current</button>
        <button on:click=move |_| {
            let _ = routine.write().skip(active.get());
        }>Skip Current</button>
        <ol class="routine">
            <For
                each=move || routine.tasks().into_iter().enumerate()
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
                                    set_active.set(Some(i));
                                    SessionStorage::set("active", i);
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
fn PreviewRoutine(
    #[prop(into)]
    routine: Field<Routine>,
) -> impl IntoView {

    view! {
        <ol class="routine">
            <For
                each=move || routine.tasks().into_iter().enumerate()
                key=|(_, task)| task.read().name.clone()
                children=move |(i, child)| {
                    view! {
                        <li class="task">
                            <TaskListItem task=child.clone() />
                        </li>
                    }
                }
            />
        </ol>
    }
}

// TODO I need a better way to set the current routine
#[component]
fn Upload(
    #[prop(into)]
    data: Field<Routine>,
) -> impl IntoView {
    let preview_routine = Store::new(Routine::default());
    view! {
        // TODO make this a form
        <h1>Upload Routine</h1>
        <input type="file" accept=".routine"
            on:change=move |ev| {
                // TODO handle the errors
                let target = ev.target().unwrap();
                let input = target.dyn_ref::<HtmlInputElement>().unwrap();
                let blob: GlooBlob = input.files()
                    .and_then(|files| files.item(0))
                    .unwrap().into();
                spawn_local(async move {
                    let contents = gloo_file::futures::read_as_text(&blob).await;
                    let tasks = routine::parse::from_csv(contents.unwrap().as_bytes());
                    let routine = Routine::with_tasks(tasks.unwrap());
                    preview_routine.set(routine);
                });
            }/>
        <button
            on:click= move |_| {
                data.set(preview_routine.get());
        }>
            Overwrite Active Routine
        </button>
        <h2>Preview</h2>
        <PreviewRoutine routine=preview_routine/>
    }
}

#[component]
fn App() -> impl IntoView {
    let data = if let Ok(list) = SessionStorage::get("in-progress-routine") {
        Store::new(list)
    } else {
        let mut list = Routine::default();
        list.push(Task::new("shower", 120));
        list.push(Task::new("eat dinner", 60));
        list.push(Task::new("program", 9990));
        Store::new(list)
    };

    let initial_active = if let Ok(i) = SessionStorage::get("active") {
        i
    } else {
        0
    };
    let (active, set_active) = signal(Some(initial_active));

    leptos::leptos_dom::helpers::set_interval(
        move || {
            // elapse time
            data.write().elapse(active.get(), Duration::from_secs(1));
            // save to session storage (TODO? when do I actually want to do this?)
            SessionStorage::set("in-progress-routine", data.get());
        },
        Duration::from_secs(1),
    );

    view! {
        <Router>
            <nav>
                <A href="">Routine</A>
                <A href="upload">Upload</A>
            </nav>
            <Routes fallback=|| "">
                <Route path=path!("/upload") view=move || view!{<Upload data=data/>} />
                <Route path=path!("/") view=move || 
            view!{<RoutinePlayer routine=data active=active set_active=set_active initial_active=initial_active/>
                }
            />
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
