use accordion_core::session::Session;
use leptos::prelude::*;
use leptos_router::{
	components::{A, Route, Router, Routes},
	path,
};
use reactive_stores::Field;

use crate::components::{DeadlinePicker, Picker, RoutinePlayer, SavedRoutineViewer, Upload};

#[component]
pub fn AppRouter(#[prop(into)] session: Field<Session>, initial_active: usize) -> impl IntoView {
	view! {
		<Router>
			<nav>
				<A href="">Picker</A>
				<A href="routine">Routine</A>
				<A href="upload">Upload</A>
			</nav>
			<Routes fallback=|| "">
				<Route path=path!("") view=Picker />
				<Route path=path!("/upload") view=move || view! { <Upload data=session /> } />
				<Route
					path=path!("/routine")
					view=move || {
						view! {
							<DeadlinePicker />
							<RoutinePlayer session=session initial_active=initial_active />
						}
					}
				/>
				<Route
					path=path!("/routine/:name")
					view=move || {
						view! { <SavedRoutineViewer data=session /> }
					}
				/>
			</Routes>
		</Router>
	}
}
