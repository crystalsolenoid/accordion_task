use std::time::Duration;

use accordion_core::routine::template::{
    RoutineTemplate, RoutineTemplateStoreFields, TaskTemplate, TaskTemplateStoreFields,
};
use leptos::prelude::*;
use leptos::{IntoView, component};
use reactive_stores::{Field, StoreFieldIterator};

use crate::components::DurationCmp;

#[component]
pub fn PreviewRoutine(#[prop(into)] routine: Field<RoutineTemplate>) -> impl IntoView {
    let (edit, set_edit) = signal(None::<usize>);
    view! {
        <table>
        <thead>
            <tr>
                <th>Name</th>
                <th>Starting Duration</th>
                <th>(Debug) ID</th>
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
                        <button on:click = move |_| {
                            routine.write().move_task_sooner(i.get());
                            routine.write();
                        }>
                        {{task.name().get()}}
                        </button>
                        </td>
                        <td>
                        <DurationCmp value=task.duration().with(|d| Duration::from_secs(*d)) />
                        </td>
                        <td>
                        {{task.id().get()}}
                        </td>
                    </tr>
                }
            }
        />
        </tbody>
        </table>
    }
}
