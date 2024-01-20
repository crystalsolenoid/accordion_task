use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::*,
};
use std::time::Duration;

use crate::app::{self, App};

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(5)])
        .split(f.size());
    render_timer(app, f, layout[0]);
    render_table(app, f, layout[1]);
}

fn render_timer(app: &mut App, f: &mut Frame, area: Rect) {
    let block = standard_block("Timer");
    let guage = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .block(block)
        .ratio(0.4);
    f.render_widget(guage, area)
}

fn render_table(app: &mut App, f: &mut Frame, area: Rect) {
    let elapsed = format_duration(app.get_time_elapsed());

    let block = standard_block("Routine");
    let rows: Vec<Row> = app
        .tasks
        .items
        .iter()
        .map(|i| generate_task_row(i))
        .collect();
    let widths = [
        Constraint::Length(5),
        Constraint::Length(25),
        Constraint::Length(15),
        Constraint::Length(15),
    ];
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .style(Style::new().fg(Color::Yellow))
        .header(
            Row::new(vec!["", "Task", "Duration", "Remaining"])
                .style(Style::new().add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
        .block(block)
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ");
    f.render_stateful_widget(table, area, &mut app.tasks.state);
}

fn format_duration(dur: Duration) -> String {
    let s = dur.as_secs();
    let m = s / 60;
    let h = m / 60;
    let h_str = match h {
        0 => "".to_string(),
        _ => format!("{}h ", h),
    };
    let m_str = match m {
        0 => "".to_string(),
        _ => format!("{}m ", m - 60 * h),
    };
    let s_str = match s {
        0 => "0s".to_string(),
        _ => format!("{}s", s - 60 * m - 60 * 60 * h),
    };
    format!("{}{}{}", h_str, m_str, s_str)
}

fn generate_task_row(task: &app::Task) -> Row {
    let checkbox = match task.complete {
        true => "[x]",
        false => "[ ]",
    }
    .to_string();
    let title = format!("{}", task.title);
    let duration = format_duration(task.timer.duration);
    let remaining = format_duration(task.get_remaining_time());
    Row::new(vec![checkbox, title, duration, remaining])
}

fn standard_block<'a>(title: &'a str) -> Block<'a> {
    Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .style(Style::new().fg(Color::Yellow))
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 1, 1))
}
