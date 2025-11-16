use accordion_core::routine::template::{RoutineTemplate, RoutineTemplateStoreFields};
use gloo_storage::{LocalStorage, Storage};
use leptos::{html, prelude::*};
use leptos_router::components::A;
use reactive_stores::{Store, StoreFieldIterator};
use web_sys::SubmitEvent;

use crate::local_storage::{StoredRoutines, StoredRoutinesStoreFields};

#[component]
pub fn Picker() -> impl IntoView {
    let routines: StoredRoutines = LocalStorage::get("stored-routines").unwrap_or_default();
    let routines_store: Store<StoredRoutines> = Store::new(routines);

    let new_routine_name: NodeRef<html::Input> = NodeRef::new();

    Effect::new(move |_| {
        LocalStorage::set("stored-routines", routines_store.get());
    });

    let (last_delete, set_last_delete) = signal(None::<RoutineTemplate>);

    view! {
        "Pick a routine."
        <form on:submit=move |ev: SubmitEvent| {
            ev.prevent_default();
            let name = new_routine_name.get()
                .expect("<input> should be mounted")
                .value();
            routines_store.vec_field().write().push(RoutineTemplate::new(name, vec![]));
        }>
            <label>
                <span>"Name"</span>
                <input type="text"
                    node_ref=new_routine_name
                    required
                />
            </label>
            <button>
                "Create New Routine"
            </button>
        </form>
        <ul class="routine-picker">
        <ForEnumerate
            each=move || routines_store.vec_field().iter_unkeyed()
            key=|routine| routine.name().get()
            children=move |i, routine| {
                view!{
                <li>
                <A href="routine/".to_string()+&routine.name().get()>{{routine.name()}}</A>
                <button on:click = move |_| {
                    let old_routine = routine.get(); // TODO cloning broken
                    let new_name = routine.name().get() + "cloned";
                    let mut new_routine = RoutineTemplate::new(new_name, vec![]);
                    old_routine.tasks.iter().for_each(|task| new_routine.push(task.clone()));
                    routines_store.vec_field().write().push(new_routine);
                }>clone</button>
                <button on:click = move |_| {
                    set_last_delete.set(Some(routines_store.vec_field().write().remove(i.get())));
                }>delete</button>
                </li>
            }
        }/>
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
