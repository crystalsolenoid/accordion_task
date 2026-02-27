use accordion_core::routine::template::{RoutineTemplate, RoutineTemplateStoreFields};
use leptos::prelude::*;

use gloo_file::Blob as GlooBlob;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::HtmlInputElement;
use reactive_stores::{Field, Store};

use accordion_core::routine;
use accordion_core::session::Session;

use crate::components::PreviewRoutine;
use crate::local_storage::{save_routine, save_session};

// TODO I need a better way to set the current routine
#[component]
pub fn Upload(#[prop(into)] data: Field<Session>) -> impl IntoView {
	let preview_routine = Store::new(RoutineTemplate::default());
	let (name, set_name) = signal("New Routine".to_string());
	view! {
		// TODO make this a form
		<h1>Upload Routine</h1>
		<input
			type="file"
			accept=".routine"
			on:change=move |ev| {
				let target = ev.target().unwrap();
				let input = target.dyn_ref::<HtmlInputElement>().unwrap();
				let blob: GlooBlob = input.files().and_then(|files| files.item(0)).unwrap().into();
				spawn_local(async move {
					let contents = gloo_file::futures::read_as_text(&blob).await;
					let mut routine = routine::parse::from_csv(contents.unwrap().as_bytes())
						.unwrap();
					routine.name = name.get();
					preview_routine.set(routine);
				});
			}
		/>
		<Show when=move || { preview_routine.tasks().read().len() != 0 }>
			<button on:click=move |_| {
				let new_session = Session::new(preview_routine.get());
				data.set(new_session);
				save_session(data);
			}>Overwrite Active Routine</button>

			<label>
				<span>"Routine Name"</span>
				<input
					id="routine-name"
					on:input:target=move |ev| {
						let routine_name = ev.target().value();
						set_name.set(routine_name.clone());
						preview_routine.name().set(routine_name);
					}
					prop:value=name
				/>
			</label>

			<button on:click=move |_| {
				save_routine(preview_routine, name);
			}>Save Routine</button>

			<h2>{{ move || preview_routine.name().get() }} <em>(Preview)</em></h2>
			<PreviewRoutine routine=preview_routine />
		</Show>
	}
}
