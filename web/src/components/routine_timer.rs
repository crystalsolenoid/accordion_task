use accordion_core::session::{Session, SessionStoreFields};
use leptos::prelude::*;
use reactive_stores::Field;

use crate::{components::DurationCmp, config::TIME_FORMAT};

#[component]
pub fn RoutineTimer(#[prop(into)] session: Field<Session>) -> impl IntoView {
    let elapsed = Memo::new(move |_| session.tasks().read().elapsed());
    let remaining = Memo::new(move |_| session.tasks().read().remaining());
    let eta = move || {
        session
            .read()
            .get_projected_end_time()
            .format(TIME_FORMAT)
            .to_string()
    };
    view! {
        <p>Remaining: <DurationCmp value=remaining /></p>
        <p>Elapsed: <DurationCmp value=elapsed /></p>
        <p>Expected End: {{ move || eta() }}</p>
    }
}
