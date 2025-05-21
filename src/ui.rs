use ratatui::{
    layout::Flex,
    prelude::{Constraint::*, *},
    style::{Color, Modifier, Style},
    widgets::*,
};
use std::time::Duration;

use crate::app::{list_pointer::ListPointer, App, Menu, Mode};
use crate::routine::{CompletionStatus, Task};

pub fn render(app: &App, f: &mut Frame) {
    match &app.help_menu {
        true => render_help_menu(f),
        false => {
            let layout = generate_layout(app, f);
            match &app.menu_focus {
                Mode::Navigation => render_timer(app, f, layout[0]),
                Mode::Typing(menu) => render_text_field(*menu, app, f, layout[0]),
            }
            render_table(app, f, layout[1]);
            if app.debug {
                render_debug(app, f, layout[2]);
            }
        }
    }
}

fn render_text_field(menu: Menu, app: &App, f: &mut Frame, area: Rect) {
    let label = match menu {
        Menu::InsertTask => "Insert New Task",
        Menu::AppendTask => "Append New Task",
        Menu::Pause => "Paused",
    };
    let mut para = app.text_input.clone();
    para.set_block(standard_block(label));
    f.render_widget(&para, area);
}

fn render_help_menu(f: &mut Frame) {
    let block = standard_block("Help");
    let para = help_paragraph()
        .style(Style::new().fg(Color::Yellow))
        .block(block);
    f.render_widget(para, f.area());
}

fn help_paragraph() -> Paragraph<'static> {
    // TODO add scroll
    let text = "Enter : Complete
S : Skip

J, K : Navigation
J + Shift,
K + Shift : Navigation (skip done)

I : Insert New Task
A : Append New Task
P : Pause (and submit message for log)
....In input mode:
....Enter : Submit
....Esc : Discard

? : Help Menu
D : Debug Panel

Q : Quit Accordion Task
";
    Paragraph::new(text).wrap(Wrap { trim: true })
}

fn generate_layout(app: &App, f: &Frame) -> [Rect; 3] {
    let width = f.area().width;
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
        .split(f.area());
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

fn render_timer(app: &App, f: &mut Frame, area: Rect) {
    let layout = Layout::horizontal([Max(7), Fill(1), Max(7)]).flex(Flex::Start);
    let block = standard_block("Timer");
    let inner = block.inner(area);

    let [a, b, c] = layout.areas(inner);

    let guage = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .ratio(app.get_percentage_elapsed());
    f.render_widget(guage, b);

    let start_time = format!("{}", app.get_start_time().format("%l:%M"));
    let start_time = Paragraph::new(start_time).block(Block::new().padding(Padding::horizontal(1)));
    f.render_widget(start_time, a);

    let end_time = format!("{}", app.get_projected_end_time().format("%l:%M"));
    let end_time = Paragraph::new(end_time).block(Block::new().padding(Padding::horizontal(1)));
    f.render_widget(end_time, c);

    f.render_widget(block, area);
}

fn render_table(app: &App, f: &mut Frame, area: Rect) {
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
    let mut state = prepare_table_state(app.task_widget_state, &block.inner(area));
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .style(Style::new().fg(Color::Yellow))
        .header(
            Row::new(vec!["", "Task", "Duration", "Remaining"])
                .style(Style::new().add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
        .block(block)
        .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ");
    f.render_stateful_widget(table, area, &mut state);
}

fn prepare_table_state(pointer: ListPointer, area: &Rect) -> TableState {
    let selected = pointer.selected().unwrap_or(0);
    // TODO can header_height be calculated? It comes from
    // the header row plus the bottom_margin of the table
    let header_height = 2;
    let height: usize = (area.height.saturating_sub(header_height)).into();
    let buffer = match height {
        // Reasoning for values:
        // We have to be able to see the current task. If we have extra room, we want to
        // first be able to see tasks after the current task (for planning). Then, when
        // the space grows, ramp up to giving a little buffer so that the previous one or
        // two tasks are shown, without violating the next-task-is-visible constraint.
        // TODO should be configurable what the maximum buffer is.
        0..=2 => 0,
        3..=4 => 1,
        5.. => 2,
    };
    let length = pointer.length();
    let max_offset = length.saturating_sub(height);
    let min_offset = selected.saturating_sub(buffer);
    let offset = min_offset.clamp(0, max_offset);
    let state: TableState = pointer.into();
    state.with_offset(offset)
}

// TODO move to utility module
pub fn format_duration(dur: Duration) -> String {
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

fn generate_task_row(task: &Task) -> Row {
    let checkbox = match task.status {
        CompletionStatus::Done => "[x]",
        CompletionStatus::NotYet => "[ ]",
        CompletionStatus::Skipped => "[-]",
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
