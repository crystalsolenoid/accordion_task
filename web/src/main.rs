// TODO track time with chrono
// sessionstorage to protect from refresh

use leptos::prelude::*;
use std::time::Duration;

use reactive_stores::{Store, Field};
use accordion_core::routine::RoutineStoreFields;
use accordion_core::routine::task::TaskStoreFields;

use accordion_core::routine::{Routine, Task, CompletionStatus};
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
fn App() -> impl IntoView {
    let mut list = Routine::default();
    list.push(Task::new("shower", 120));
    list.push(Task::new("eat dinner", 60));
    list.push(Task::new("program", 9990));
    let data = Store::new(list);

    let (active, set_active) = signal(Some(0));

    leptos::leptos_dom::helpers::set_interval(
        move || data.write().elapse(active.get(), Duration::from_secs(1)),
        Duration::from_secs(1),
    );

    view! {
        <RoutineTimer routine=data />
        <button on:click=move |_| {
            let _ = data.write().toggle(active.get());
        }>Complete Current</button>
        <button on:click=move |_| {
            let _ = data.write().skip(active.get());
        }>Skip Current</button>
        <ol class="routine">
            <For
                each=move || data.tasks().into_iter().enumerate()
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
                                prop:checked=i == 0
                                on:change=move |_| set_active.set(Some(i))
                            />
                        </li>
                    }
                }
            />
        </ol>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
