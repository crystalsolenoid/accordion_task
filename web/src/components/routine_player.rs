use accordion_core::{
	routine::{RoutineStoreFields, task::TaskStoreFields},
	session::{Session, SessionStoreFields},
};
use gloo_storage::{SessionStorage, Storage};
use leptos::{html, prelude::*};
use reactive_stores::Field;
use web_sys::{FocusOptions, wasm_bindgen::JsCast};

use crate::components::{RoutineTimer, TaskListItem};

fn focus_by_id(id: &str) {
	let mut focus_options = FocusOptions::new();
	focus_options.set_focus_visible(true);
	document()
		.get_element_by_id(id)
		.unwrap()
		.dyn_into::<web_sys::HtmlElement>()
		.unwrap()
		.focus_with_options(&focus_options)
		.unwrap();
}

#[component]
pub fn RoutinePlayer(
	#[prop(into)] session: Field<Session>,
	initial_active: usize,
) -> impl IntoView {
	let active = RwSignal::new(initial_active.to_string());
	view! {
		<a id="test-focus" href="/">
			test
		</a>
		<div id="task-actions">
			<button on:click=move |_| {
				focus_by_id("label-3");
			}>Focus test</button>
			<RoutineTimer session=session />
			<button on:click=move |_| {
				let _ = session.write().toggle_advance();
				let i = session.read().selected.selected();
				if let Some(i) = i {
					active.set(i.to_string());
				}
				let _ = SessionStorage::set("in-progress-session", session.get());
				let task_id = session.read().get_selected_task().unwrap().id;
				focus_by_id(&format!("label-{}", task_id));
			}>Complete Current</button>
			<button on:click=move |_| {
				let _ = session.write().skip_advance();
				let i = session.read().selected.selected();
				if let Some(i) = i {
					active.set(i.to_string());
				}
				let _ = SessionStorage::set("in-progress-session", session.get());
				let task_id = session.read().get_selected_task().unwrap().id;
				focus_by_id(&format!("label-{}", task_id));
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
							// <a href="/" id=i>{i}</a>
							// <input id=i/>
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
