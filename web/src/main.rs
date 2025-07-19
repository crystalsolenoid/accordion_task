use leptos::prelude::*;
use std::time::Duration;

use accordion_core::routine::{Routine, Task};
use accordion_core::utils;

#[component]
fn Duration(value: ReadSignal<Duration>) -> impl IntoView {
    view! {
        {{ utils::format_duration(value.get()) }}
    }
}

#[component]
fn Routine(
    name: RwSignal<String>,
    elapsed: ReadSignal<Duration>,
    duration: ReadSignal<Duration>,
) -> impl IntoView {
    view! {
        <h2>{{ name }}</h2>: <Duration value=elapsed/> / <Duration value=duration/>
        <progress value=move || elapsed.get().as_secs() as f64 / duration.get().as_secs() as f64 />
    }
}

#[component]
fn App() -> impl IntoView {
    let mut list = Routine::default();
    list.push(Task::new("a", 120));
    list.push(Task::new("b", 60));

    let myTask = Task::new("wash dishes", 120);
    let name = RwSignal::new("wash dishes".to_string());
    let elapsed = RwSignal::new(Duration::from_secs(40));
    let duration = RwSignal::new(Duration::from_secs(120));

    view! {
        <Routine name=name elapsed=elapsed.read_only() duration=duration.read_only()/>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
