use accordion_core::{
	routine::{RoutineStoreFields, task::TaskStoreFields},
	session::{Session, SessionStoreFields},
};
use gloo_storage::{SessionStorage, Storage};
use leptos::prelude::*;
use reactive_stores::Field;

use crate::components::{RoutineTimer, TaskListItem};

#[component]
pub fn RoutinePlayer(
	#[prop(into)] session: Field<Session>,
	initial_active: usize,
) -> impl IntoView {
	let active = RwSignal::new(initial_active.to_string());
	view! {
		<div id="task-actions">
			<RoutineTimer session=session />
			<button on:click=move |_| {
				let _ = session.write().toggle_advance();
				let i = session.read().selected.selected();
				if let Some(i) = i {
					active.set(i.to_string());
				}
				let _ = SessionStorage::set("in-progress-session", session.get());
			}>Complete Current</button>
			<button on:click=move |_| {
				let _ = session.write().skip_advance();
				let i = session.read().selected.selected();
				if let Some(i) = i {
					active.set(i.to_string());
				}
				let _ = SessionStorage::set("in-progress-session", session.get());
			}>Skip Current</button>
		</div>
		<ol class="routine">
			<For
				each=move || session.tasks().tasks().into_iter().enumerate()
				key=|(_, task)| task.read().name.clone()
				children=move |(i, child)| {
					view! {
						<li class="task">
							<TaskListItem task=child />
							<input
								type="radio"
								class="active-task"
								value=i
								id=child.name()
								name="active"
								bind:group=active
								on:change=move |_| {
									let _ = session.selected().write().select(Some(i));
									let _ = SessionStorage::set(
										"in-progress-session",
										session.get(),
									);
								}
							/>
						</li>
					}
				}
			/>
		</ol>
	}
}
