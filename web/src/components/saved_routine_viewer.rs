use accordion_core::{
	routine::template::RoutineTemplateStoreFields,
	session::Session,
	utils::{seconds_to_eta, seconds_to_start},
};
use chrono::{Local, NaiveTime};
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use leptos::{
	html,
	leptos_dom::{debug_log, helpers},
	prelude::*,
};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use reactive_stores::{Field, Store, StoreFieldIterator};

use crate::{
	components::PreviewRoutine,
	config::TIME_FORMAT,
	local_storage::{StoredRoutines, StoredRoutinesStoreFields},
};

// TODO rename
#[derive(Params, PartialEq)]
struct ContactParams {
	name: Option<String>,
}

#[component]
pub fn SavedRoutineViewer(#[prop(into)] data: Field<Session>) -> impl IntoView {
	let params = use_params::<ContactParams>();
	let name = move || {
		params
			.read()
			.as_ref()
			.ok()
			.and_then(|params| params.name.clone()) // TODO lazy clone
			.unwrap_or_default()
	};

	let routines: StoredRoutines = LocalStorage::get("stored-routines").unwrap_or_default();
	let routines_store: Store<StoredRoutines> = Store::new(routines);

	Effect::new(move |_| {
		LocalStorage::set("stored-routines", routines_store.get());
	});

	let deadline: NodeRef<html::Input> = NodeRef::new();

	let routine_store = move || {
		routines_store
			.vec_field()
			.iter_unkeyed()
			.find(|rs| rs.name().get() == name())
			.unwrap()
	};

	let (now, set_now) = signal(Local::now());

	helpers::set_interval(
		move || {
			set_now.set(Local::now());
		},
		std::time::Duration::from_secs(20),
	);

	let eta = Memo::new(move |_| {
		seconds_to_eta(now.get(), routine_store().read().total_duration())
			.format(TIME_FORMAT)
			.to_string()
	});

	let start_by = Memo::new(move |_| {
		if let Some(deadline) = routine_store().read().conf_get_default_deadline() {
			seconds_to_start(now.get(), deadline, routine_store().read().total_duration())
				.format(TIME_FORMAT)
				.to_string()
		} else {
			"".to_string()
		}
	});

	view! {
		<h1>{{ move || routine_store().name() }}</h1>
		{{
			move || {
				if let Some(deadline) = routine_store().read().conf_get_default_deadline() {
						deadline.to_string()
					} else {
						"no deadline".to_string()
					}
				}
		}}
		<button on:click=move |_| {
			routine_store()
				.write()
				.conf_set_default_deadline(
					NaiveTime::parse_from_str(&deadline.get().unwrap().value(), "%H:%M").unwrap(),
				);
		}>"set default deadline"</button>
		<label>
			<span>"Default Deadline"</span>
			<input node_ref=deadline id="deadline" type="time" name="deadline" />
		</label>
		<p>"Recommended start time: " {{ move || start_by.get() }}</p>
		<p>"Projected end time: " {{ move || eta.get() }}</p>
		// TODO instead, link to a page
		// thats for that routine? Maybe?
		<button on:click=move |_| {
			let new_session = Session::new(routine_store().get());
			data.set(new_session);
			SessionStorage::set("in-progress-session", data.get());
		}>Overwrite Active Routine</button>
		<PreviewRoutine routine=routine_store() />
		{move || routine_store().tasks().get().len()}
	}
}
