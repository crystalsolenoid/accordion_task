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
