use ratatui::{
  prelude::{*},
  style::{Color, Style, Modifier},
  widgets::{*},
};

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
  let block = standard_block("Routine");
  let rows = [
    generate_task_row(&app.tasks.items[0]),
    generate_task_row(&app.tasks.items[1]),
    generate_task_row(&app.tasks.items[2]),
  ];
  let widths = [
      Constraint::Length(5),
      Constraint::Length(25),
      Constraint::Length(15),
  ];
  let table = Table::new(rows, widths)
    .column_spacing(1)
    .style(Style::new().fg(Color::Yellow))
    .header(
      Row::new(vec!["", "Task", "Duration"])
        .style(Style::new().add_modifier(Modifier::BOLD))
        .bottom_margin(1),
    )
    .block(block)
    .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");
  f.render_stateful_widget(
    table,
    area,
    &mut app.tasks.state
  );
}

fn generate_task_row(task: &app::Task) -> Row {
  let checkbox = match task.complete {
    true => "[x]",
    false => "[ ]"
  }.to_string();
  let title = format!("{}", task.title);
  let duration = format!("{}", task.dur);
  Row::new(vec![checkbox, title, duration])
}

fn standard_block<'a>(title: &'a str) -> Block<'a> {
  Block::default()
    .title(title)
    .title_alignment(Alignment::Center)
    .style(Style::new().fg(Color::Yellow))
    .borders(Borders::ALL)
    .padding(Padding::new(2, 2, 1, 1))
}
