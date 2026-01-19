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
pub fn DownloadRoutine(#[prop(into)] routine: Field<RoutineTemplate>) -> impl IntoView {
    let (edit, set_edit) = signal(None::<usize>);
    // let (download, set_download) = signal("data:text/html;charset=utf-8,unset");
    let download = { move || format!("data:text/html;charset=utf-8,{}", routine.get().get_routine_file()) };
    view! {
        /*
    {
        move || routine.get().tasks.len()
    }
*/
        <a href=download() download="download.txt">Download here!</a>
    }
}
