use accordion_core::{routine::template::RoutineTemplate, session::Session};
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use leptos::prelude::{Get, ReadSignal};
use reactive_stores::{Field, Store};
use serde::{Deserialize, Serialize};

// TODO move ALL usage of gloo_storage into
// this module

#[derive(Default, Store, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StoredRoutines {
	pub vec_field: Vec<RoutineTemplate>,
}

pub fn save_session(session: impl Into<Field<Session>>) {
	let session: Field<Session> = session.into();
	let _ = SessionStorage::set("in-progress-session", session.get());
}

pub fn save_routine(routine: Store<RoutineTemplate>, name: ReadSignal<String>) {
	let mut stored_routines: StoredRoutines =
		LocalStorage::get("stored-routines").unwrap_or_default();
	let already_exists = stored_routines
		.vec_field
		.iter()
		.find(|r| r.name == name.get())
		.is_some();
	if !already_exists {
		stored_routines.vec_field.push(routine.get());
		let _ = LocalStorage::set("stored-routines", stored_routines);
	}
}

pub fn save_routine_list(routines: Store<StoredRoutines>) {
	let _ = LocalStorage::set("stored-routines", routines.get());
}
