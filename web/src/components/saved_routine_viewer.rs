use accordion_core::{routine::template::RoutineTemplateStoreFields, session::Session};
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use reactive_stores::{Field, Store, StoreFieldIterator};

use crate::{
    components::PreviewRoutine,
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

    let routine_store = move || {
        routines_store
            .vec_field()
            .iter_unkeyed()
            .find(|rs| rs.name().get() == name())
            .unwrap()
    };

    view! {
        <h1>
            {{move || routine_store().name()}}
        </h1>
        // TODO instead, link to a page
        // thats for that routine? Maybe?
        <button on:click=move |_| {
            let new_session = Session::new(routine_store().get());
            data.set(new_session);
            SessionStorage::set("in-progress-session", data.get());
        }>Overwrite Active Routine</button>
        <PreviewRoutine routine=routine_store() />
    }
}
