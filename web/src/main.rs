use leptos::prelude::*;

use accordion_core::routine::Task;

#[component]
fn App() -> impl IntoView {
    let myTask = Task::new("wash dishes", 120);

    view! {
        <p>{{ myTask.name }}</p>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
