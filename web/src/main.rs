use leptos::prelude::*;
use std::time::Duration;

use reactive_stores::{Store, StoreFieldIter, StoreFieldIterator};
use reactive_stores::StoreField;
use reactive_stores::Field;
//use reactive_graph::traits::Read;
//use reactive_graph::traits::Get;
use accordion_core::routine::RoutineStoreFields;
use accordion_core::routine::task::TaskStoreFields;

use accordion_core::routine::{Routine, Task};
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
        <h2>{{ move || task.name().get() }}</h2> <Duration value=task.elapsed() /> / <Duration value=task.duration() />
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

    leptos::leptos_dom::helpers::set_interval(
        move || data.update(|d| d.elapse(Some(0), Duration::from_secs(1))),
        Duration::from_secs(1),
    );

    view! {
    <ol>
        <For
            each=move || data.tasks()
            key=|task| task.read().name.clone()
                children=|child| {
                view! {
                    <li>
                    <TaskListItem task=child/>
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
