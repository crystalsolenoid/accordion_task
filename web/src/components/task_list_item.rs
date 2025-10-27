use accordion_core::routine::{CompletionStatus, Task, task::TaskStoreFields};
use leptos::prelude::*;
use reactive_stores::Field;

use crate::components::DurationCmp;

#[component]
pub fn TaskListItem(#[prop(into)] task: Field<Task>) -> impl IntoView {
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
                <DurationCmp value=task.elapsed() />
            </span>
            <progress value=move || {
                task.elapsed().get().as_secs() as f64 / task.duration().get().as_secs() as f64
            } />
            <span>
                <DurationCmp value=task.duration() />
            </span>
        </div>
    }
}
