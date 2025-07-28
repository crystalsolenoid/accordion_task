use leptos::prelude::*;
use std::time::Duration;

use reactive_stores::{Store, StoreFieldIter, StoreFieldIterator};
use reactive_stores::StoreField;
use reactive_stores::Field;
//use reactive_graph::traits::Read;
//use reactive_graph::traits::Get;
use accordion_core::routine::RoutineStoreFields;
use accordion_core::routine::task::TaskStoreFields;

use accordion_core::routine::{Routine, Task, CompletionStatus};
use accordion_core::utils;

#[component]
fn Duration(
    #[prop(into)]
    value: Signal<Duration>) -> impl IntoView {
    view! {
        {{ move || utils::format_duration(value.get()) }}
    }
}

#[component]
fn TaskListItem(
    #[prop(into)]
    task: Field<Task>,
) -> impl IntoView {
    view! {
        <h2>{{ move || task.name().get() }}</h2> <p>
        {{ move || match task.status().get() {
                CompletionStatus::NotYet => "Incomplete",
                CompletionStatus::Done => "Done",
                CompletionStatus::Skipped => "Skipped",
    } }}
            </p>
            <Duration value=task.elapsed() /> / <Duration value=task.duration() />
        <progress value=move || task.elapsed().get().as_secs() as f64 / task.duration().get().as_secs() as f64 />
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
        move || data.update(|d| d.elapse(active.get(), Duration::from_secs(1))),
        Duration::from_secs(1),
    );

    view! {
    <button
        on:click=move |_| {data.write().toggle(active.get());}
        >Complete Current</button>
    <button
        on:click=move |_| {data.write().skip(active.get());}
        >Skip Current</button>
    <ol>
        <For
            each=move || data.tasks().into_iter().enumerate()
            key=|(_, task)| task.read().name.clone()
                children=move |(i, child)| {
                view! {
                    <li>
                    <TaskListItem task=child.clone()/>
                    <input type="radio" value=i id=child.name() name="active" on:change=move |_| set_active.set(Some(i))/>
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
