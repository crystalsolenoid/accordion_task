use leptos::prelude::*;
use leptos_router::{components::Form, hooks::use_query};

use crate::config::TIME_FORMAT;
use crate::router::DeadlineQuery;

#[component]
pub fn DeadlinePicker() -> impl IntoView {
	view! {
		<Form method="GET" action="">
			<label>
				<span>"Set Deadline"</span>
				<input type="time" name="d" />
			</label>
			<button>Submit</button>
		</Form>
		{{ move || use_query::<DeadlineQuery>()
			.read()
			.as_ref()
			.ok()
			.and_then(|queries| queries.d)
			.map(|d| d.format(TIME_FORMAT).to_string())
			 }}
	}
}
