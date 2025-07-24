use leptos::prelude::*;
use std::time::Duration;

use reactive_stores::{Store, StoreFieldIter, StoreFieldIterator};
use reactive_stores::StoreField;
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
fn Routine(
    name: RwSignal<String>,
    elapsed: ReadSignal<Duration>,
    duration: ReadSignal<Duration>,
) -> impl IntoView {
    view! {
        <h2>{{ name }}</h2> <Duration value=elapsed/> / <Duration value=duration/>
        <progress value=move || elapsed.get().as_secs() as f64 / duration.get().as_secs() as f64 />
    }
}

#[component]
fn App() -> impl IntoView {
    let mut list = Routine::default();
    list.push(Task::new("shower", 120));
    list.push(Task::new("eat dinner", 60));
    let data = Store::new(list);

    let myTask = Task::new("wash dishes", 120);

    let name = RwSignal::new("wash dishes".to_string());
    let elapsed = RwSignal::new(Duration::ZERO);
    let duration = RwSignal::new(Duration::from_secs(120));

    leptos::leptos_dom::helpers::set_interval(
        move || elapsed.update(|n| *n += Duration::from_secs(1)),
        Duration::from_secs(1),
    );

    leptos::leptos_dom::helpers::set_interval(
        move || data.update(|d| d.elapse(Some(0), Duration::from_secs(1))),
        Duration::from_secs(1),
    );

    view! {
        <Routine name=name elapsed=elapsed.read_only() duration=duration.read_only()/>
    <p>Name: {{move || data.tasks().at_unkeyed(0).name().get()}}</p>
    <p>Duration: <Duration value=data.tasks().at_unkeyed(0).duration()/></p>
    <p>Elapsed: <Duration value=data.tasks().at_unkeyed(0).elapsed()/></p>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
