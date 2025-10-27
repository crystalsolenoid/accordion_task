use chrono::NaiveTime;
use leptos::Params;
use leptos_router::params::Params;

#[derive(Params, PartialEq)]
pub struct DeadlineQuery {
    pub d: Option<NaiveTime>,
}
