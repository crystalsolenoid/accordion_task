use std::time::Duration;

use accordion_core::routine::task::parse_new::parse_duration;
use accordion_core::routine::template::{
    RoutineTemplate, RoutineTemplateStoreFields, TaskTemplate, TaskTemplateStoreFields,
};
use leptos::leptos_dom::debug_log;
use leptos::{IntoView, component};
use leptos::{html, prelude::*};
use reactive_stores::{Field, StoreFieldIterator};
use web_sys::SubmitEvent;

use crate::components::DurationCmp;

const DURATION_VALIDATOR: &str = "(([0-9]+m)([0-9]+s)?)|(([0-9]+m)?([0-9]+s))";

#[component]
pub fn PreviewRoutine(#[prop(into)] routine: Field<RoutineTemplate>) -> impl IntoView {
    let (edit, set_edit) = signal(None::<usize>);
    let new_task_name: NodeRef<html::Input> = NodeRef::new();
    let new_task_duration: NodeRef<html::Input> = NodeRef::new();
    Effect::new(move |_| {
        debug_log!("routine updated {}", routine.get().tasks.len());
    });
    Effect::new(move |_| {
        debug_log!("routine tasks updated {}", routine.tasks().get().len());
    });
    view! {
    {
        move || routine.get().tasks.len()
    }
        <form on:submit=move |ev: SubmitEvent| {
            ev.prevent_default();
            let name = new_task_name.get()
                .expect("<input> should be mounted")
                .value();
            let raw_duration = new_task_duration.get()
                .expect("<input> should be mounted")
                .value();
            let duration = parse_duration(&raw_duration);
            match duration {
                Ok(d) => {routine.update(|routine| routine.push(TaskTemplate::new(&name, d, 0)));
                    routine.tasks().update(|_| ());
                    // TODO this is a sad hack. Tasks should update on its own.
                },
                Err(_) => (),
            };
        }>
        <label>
            <span>"Name"</span>
            <input type="text"
                node_ref=new_task_name
                required
            />
        </label>
        <label>
            <span>"Duration"</span>
            <input type="text"
                node_ref=new_task_duration
                pattern=DURATION_VALIDATOR
            />
        </label>
            <button>
            "Create"
            </button>
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
                view!{
                    <tr>
                        <td>
                        // <button on:click = move |_| {
                        //     routine.write().move_task_sooner(i.get());
                        // }>
                        {{task.name().get()}}
                        // </button>
                        </td>
                        <td>
                        <DurationCmp value=task.duration().with(|d| Duration::from_secs(*d)) />
                        </td>
                        <td>
                            <button on:click = move |_| {
                                routine.write().move_task_sooner(i.get());
                            }>
                            "^"
                            </button>
                            <button on:click = move |_| {
                                routine.write().move_task_later(i.get());
                            }>
                            "v"
                            </button>
                        </td>
                        <td>
                            <button on:click = move |_| {
                                routine.write().remove_task(i.get());
                            }>
                            "x"
                            </button>
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
