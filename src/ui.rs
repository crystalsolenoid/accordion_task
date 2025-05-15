use ratatui::{
    layout::Flex,
    prelude::{Constraint::*, *},
    style::{Color, Modifier, Style},
    widgets::*,
};
use std::time::Duration;

use crate::app::static_task::CompletionStatus;
use crate::app::{static_task::Task, App, Menu, Mode};

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

I: Insert New Task
A: Append New Task
....In new task mode:
....Enter: Submit
....Esc: Discard

? : Help Menu
D : Debug Panel

Q, Esc : Quit Accordion Task
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
    let mut state: TableState = app.task_widget_state.into();
    f.render_stateful_widget(table, area, &mut state);
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
