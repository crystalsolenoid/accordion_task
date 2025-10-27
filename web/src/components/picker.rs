use accordion_core::routine::template::{RoutineTemplate, RoutineTemplateStoreFields};
use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use leptos_router::components::A;
use reactive_stores::{Store, StoreFieldIterator};

use crate::local_storage::{StoredRoutines, StoredRoutinesStoreFields};

#[component]
pub fn Picker() -> impl IntoView {
    let routines: StoredRoutines = LocalStorage::get("stored-routines").unwrap_or_default();
    let routines_store: Store<StoredRoutines> = Store::new(routines);

    Effect::new(move |_| {
        LocalStorage::set("stored-routines", routines_store.get());
    });

    let (last_delete, set_last_delete) = signal(None::<RoutineTemplate>);

    view! {
        "Pick a routine."
        <ul>
        {move || routines_store.vec_field().iter_unkeyed().enumerate()
            .map(|(i, routine)| view!{
                <li>
                <A href=move || "routine/".to_string()+&routine.name().get()>{{routine.name()}}</A>
                <button on:click = move |_| {
                    let new_routine = routine.get();
                    let new_name = routine.name().get() + "cloned";
                    routines_store.vec_field().write().push(RoutineTemplate::new(new_name, vec![]));
                }>clone</button>
                <button on:click = move |_| {
                    set_last_delete.set(Some(routines_store.vec_field().write().remove(i)));
                }>delete</button>
                </li>
            })
            .collect_view()
        }
        </ul>
        <Show
            when=move || {last_delete.get().is_some()}
            fallback=|| view! {}
        >
            <button on:click=move |_| {
                routines_store.vec_field().write().push(last_delete.get().unwrap());
                set_last_delete.set(None);
            }>Restore last deleted: {move || last_delete.get().unwrap().name}</button>
        </Show>
    }
}
