use std::time::Duration;

use accordion_core::{routine::task::FlexDuration, utils};
use leptos::prelude::*;

#[component]
pub fn DurationCmp(#[prop(into)] value: Signal<Duration>) -> impl IntoView {
	view! { {{ move || utils::format_duration(&value.get()) }} }
}

#[component]
pub fn FlexDurationCmp(#[prop(into)] value: Signal<FlexDuration>) -> impl IntoView {
	view! { {{ move || utils::format_flex_duration(&value.get()) }} }
}
