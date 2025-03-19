use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::*,
};
use std::time::Duration;

use crate::app::{static_task::StaticTask, App, SignedDuration};

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = generate_layout(app, f);
    render_timer(app, f, layout[0]);
    render_table(app, f, layout[1]);
    if app.debug {
        render_debug(app, f, layout[2]);
    }
}

fn generate_layout(app: &App, f: &Frame) -> [Rect; 3] {
    let width = f.size().width;
    let height = f.size().height;
    let header_height = 5;
    let footer_height = match app.debug {
        true => 15,
        false => 0,
    };
    let split1 = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(header_height),
            Constraint::Min(header_height),
        ])
        .split(f.size());
    let header = split1[0];
    let split2 = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(width - header_height - footer_height),
            Constraint::Min(footer_height),
        ])
        .split(split1[1]);
    let body = split2[0];
    let footer = split2[1];
    [header, body, footer]
}

fn render_debug(app: &App, f: &mut Frame, area: Rect) {
    let block = standard_block("Debug");
    let text = vec![
        format!(
            "get_total_remaining() \t{}",
            format_duration(app.get_total_remaining())
        )
        .into(),
        format!(
            "get_unused_time() \t{}",
            format_duration(app.get_unused_time())
        )
        .into(),
        format!(
            "get_time_balance() \t{}",
            format_signed_duration(app.get_time_balance())
        )
        .into(),
        format!("start time \t{}", app.get_start_time().format("%l:%M%P")).into(),
        format!(
            "projected end time \t{}",
            app.get_projected_end_time().format("%l:%M%P")
        )
        .into(),
    ];
    let para = Paragraph::new(text)
        .style(Style::new().fg(Color::Yellow))
        .block(block);
    f.render_widget(para, area);
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
        .ratio(app.get_percentage_elapsed());
    f.render_widget(guage, area)
}

fn render_table(app: &mut App, f: &mut Frame, area: Rect) {
    let elapsed = format_duration(app.get_time_elapsed());

    let block = standard_block("Routine");
    let rows: Vec<Row> = app
        .tasks
        .tasks
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
    f.render_stateful_widget(table, area, &mut app.task_widget_state);
}

fn format_signed_duration(signed_dur: SignedDuration) -> String {
    match signed_dur {
        SignedDuration::DEFICIT(dur) => format!("{} behind schedule", format_duration(dur)),
        SignedDuration::SURPLUS(dur) => format!("{} ahead of schedule", format_duration(dur)),
        SignedDuration::ZERO => "0s".to_string(),
    }
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
        _ => format!("{}s", s - 60 * m),
    };
    format!("{}{}{}", h_str, m_str, s_str)
}

fn generate_task_row(task: &StaticTask) -> Row {
    let checkbox = match task.complete {
        true => "[x]",
        false => "[ ]",
    }
    .to_string();
    let title = task.name.to_string();
    let duration = format_duration(task.duration);
    let remaining = format_duration(task.remaining());
    Row::new(vec![checkbox, title, duration, remaining])
}

fn standard_block(title: &str) -> Block<'_> {
    Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .style(Style::new().fg(Color::Yellow))
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 1, 1))
}
