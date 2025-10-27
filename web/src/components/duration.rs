use std::time::Duration;

use accordion_core::utils;
use leptos::prelude::*;

#[component]
pub fn DurationCmp(#[prop(into)] value: Signal<Duration>) -> impl IntoView {
    view! {
        {{ move || utils::format_duration(value.get()) }}
    }
}
