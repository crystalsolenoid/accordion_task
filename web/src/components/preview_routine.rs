use accordion_core::routine::task::parse_new::parse_duration;
use accordion_core::routine::template::{
	RoutineTemplate, RoutineTemplateStoreFields, TaskTemplate, TaskTemplateStoreFields,
};
use leptos::leptos_dom::debug_log;
use leptos::{IntoView, component};
use leptos::{html, prelude::*};
use reactive_stores::{Field, StoreFieldIterator};
use web_sys::SubmitEvent;

use crate::components::FlexDurationCmp;

const DURATION_VALIDATOR: &str = r"((([0-9]+m)([0-9]+s)?)|(([0-9]+m)?([0-9]+s)))\+?";

#[component]
pub fn PreviewRoutine(#[prop(into)] routine: Field<RoutineTemplate>) -> impl IntoView {
	let new_task_form: NodeRef<html::Form> = NodeRef::new();
	let new_task_name: NodeRef<html::Input> = NodeRef::new();
	let new_task_duration: NodeRef<html::Input> = NodeRef::new();
	let reorder_selection = RwSignal::<Option<usize>>::new(None);
	Effect::new(move |_| {
		debug_log!("routine updated {}", routine.get().tasks.len());
	});
	Effect::new(move |_| {
		debug_log!("routine tasks updated {}", routine.tasks().get().len());
	});
	view! {
		{move || routine.get().tasks.len()}
		<form
			node_ref=new_task_form
			on:submit=move |ev: SubmitEvent| {
				ev.prevent_default();
				let name = new_task_name.get().expect("<input> should be mounted").value();
				let raw_duration = new_task_duration
					.get()
					.expect("<input> should be mounted")
					.value();
				let duration = parse_duration(&raw_duration);
				if let Ok(d) = duration {
					routine.update(|routine| routine.push(TaskTemplate::new(&name, d, 0)));
					routine.tasks().update(|_| ());
					new_task_form.get().expect("<form> should be mounted").reset();
				}
			}
		>
			<label>
				<span>"Name"</span>
				<input type="text" node_ref=new_task_name required />
			</label>
			<label>
				<span>"Duration"</span>
				<input type="text" node_ref=new_task_duration pattern=DURATION_VALIDATOR />
			</label>
			<button>"Create"</button>
		</form>
		<table>
			<thead>
				<tr>
					<th>Name</th>
					<th>Starting Duration</th>
					<th>Reorder</th>
					<th>Remove</th>
				// <th>(Debug) ID</th>
				</tr>
			</thead>
			<tbody>
				<ForEnumerate
					each=move || routine.tasks().iter_unkeyed()
					key=|task| task.id().get()
					children=move |i, task| {
						view! {
							<tr>
								<td>
									// <button on:click = move |_| {
									// routine.write().move_task_sooner(i.get());
									// }>
									{{ task.name().get() }}
								// </button>
								</td>
								<td>
									<FlexDurationCmp value=task.duration().get() />
								</td>
								<td>
									<button on:click=move |_| {
										routine.write().move_task_sooner(i.get());
									}>"^"</button>
									<button on:click=move |_| {
										if let Some(j) = reorder_selection.get() {
											routine.write().insert_task_before(j, i.get());
											reorder_selection.set(None);
										} else {
											reorder_selection.set(Some(i.get()));
										}
									}>
										{move || match reorder_selection.get() {
											Some(j) if j == i.get() => "o",
											Some(j) => "x",
											None => "o",
										}}
									</button>
									<button on:click=move |_| {
										routine.write().move_task_later(i.get());
									}>"v"</button>
								</td>
								<td>
									<button on:click=move |_| {
										routine.write().remove_task(i.get());
									}>"x"</button>
								</td>
							// <td>
							// {{task.id().get()}}
							// </td>
							</tr>
						}
					}
				/>
			</tbody>
		</table>
	}
}
