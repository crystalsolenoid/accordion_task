use accordion_core::{
    routine::{RoutineStoreFields, task::TaskStoreFields},
    session::{Session, SessionStoreFields},
};
use gloo_storage::{SessionStorage, Storage};
use leptos::prelude::*;
use reactive_stores::Field;

use crate::components::{RoutineTimer, TaskListItem};

#[component]
pub fn RoutinePlayer(
    #[prop(into)] session: Field<Session>,
    initial_active: usize,
) -> impl IntoView {
    view! {
        <div id="task-actions">
            <RoutineTimer session=session />
            <button on:click=move |_| {
                let _ = session.write().toggle();
            }>Complete Current</button>
            <button on:click=move |_| {
                let _ = session.write().skip();
            }>Skip Current</button>
        </div>
        <ol class="routine">
            <For
                each=move || session.tasks().tasks().into_iter().enumerate()
                key=|(_, task)| task.read().name.clone()
                children=move |(i, child)| {
                    view! {
                        <li class="task">
                            <TaskListItem task=child.clone() />
                            <input
                                type="radio"
                                class="active-task"
                                value=i
                                id=child.name()
                                name="active"
                                prop:checked=i == initial_active
                                on:change=move |_| {
                                    session.selected().write().select(Some(i));
                                    SessionStorage::set("in-progress-session", session.get());
                                }
                            />
                        </li>
                    }
                }
            />
        </ol>
    }
}
