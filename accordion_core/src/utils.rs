use chrono::{DateTime, Days, Local, MappedLocalTime, NaiveTime};
use std::time::Duration;

pub fn format_duration(dur: Duration) -> String {
	let s = dur.as_secs();
	let m = s / 60;
	let h = m / 60;
	let h_str = match h {
		0 => String::new(),
		_ => format!("{h}h "),
	};
	let m_str = match m {
		0 => String::new(),
		_ => format!("{}m ", m - 60 * h),
	};
	let s_str = match s {
		0 => "0s".to_string(),
		_ => format!("{}s", s - 60 * m),
	};
	format!("{h_str}{m_str}{s_str}")
}

pub fn interpret_naive_time(now: DateTime<Local>, nt: NaiveTime) -> DateTime<Local> {
	{
		// TODO handle DST
		let MappedLocalTime::Single(today_deadline) = now.with_time(nt) else {
			todo!("Handle DST");
		};
		let deadline = if today_deadline < now {
			let Some(tomorrow) = now.checked_add_days(Days::new(1)) else {
				todo!("handle DST properly")
			};
			match tomorrow.with_time(nt) {
				MappedLocalTime::Single(t) => t,
				_ => todo!(), // Risks crash around DST change
			}
		} else {
			today_deadline
		};
		deadline
	}
}

pub fn seconds_to_eta(now: DateTime<Local>, duration: u64) -> DateTime<Local> {
	now + Duration::from_secs(duration)
}
