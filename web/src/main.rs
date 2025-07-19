use leptos::prelude::*;
use std::time::Duration;

use accordion_core::routine::{Routine, Task};

#[component]
fn Routine(
    name: RwSignal<String>,
    elapsed: RwSignal<Duration>,
    duration: RwSignal<Duration>,
) -> impl IntoView {
    view! {
        {{ name }}: {{ elapsed.get().as_secs() }} / {{ duration.get().as_secs() }}
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
        <p>{{ myTask.name }}</p>
        <Routine name=name elapsed=elapsed duration=duration/>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
