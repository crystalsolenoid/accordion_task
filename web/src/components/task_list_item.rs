use accordion_core::routine::{CompletionStatus, Task, task::TaskStoreFields};
use leptos::prelude::*;
use reactive_stores::Field;

use crate::components::DurationCmp;

#[component]
pub fn TaskListItem(#[prop(into)] task: Field<Task>) -> impl IntoView {
	let plain_text_status = move || match task.status().get() {
		CompletionStatus::NotYet => "incomplete",
		CompletionStatus::Done => "done",
		CompletionStatus::Skipped => "skipped",
	};

	let status_symbol = move || match task.status().get() {
		CompletionStatus::NotYet => "☐",
		CompletionStatus::Done => "☑",
		CompletionStatus::Skipped => "☒",
	};

	view! {
		<label for=move || task.name().get() class=plain_text_status>
			<h2>
				<span class="checkbox" aria-label=plain_text_status>
					{{ status_symbol }}
				</span>
				" "
				{{ move || task.name().get() }}
			</h2>
			<div class="progress-values">
				<span class="duration left">
					<DurationCmp value=task.elapsed() />
				</span>
				<progress value=move || {
					task.elapsed().get().as_secs() as f64 / task.duration().get().as_secs() as f64
				} />
				<span class="duration right">
					<DurationCmp value=task.duration() />
				</span>
			</div>
		</label>
	}
}
