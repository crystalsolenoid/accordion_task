use ratatui::{
  prelude::{Alignment, Frame, Constraint},
  style::{Color, Style, Modifier},
  widgets::{*},
};

use crate::app::{self, App};

pub fn render(app: &mut App, f: &mut Frame) {
//  render_list(app, f);
  render_table(app, f);
}

fn render_table(app: &mut App, f: &mut Frame) {
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
      Row::new(vec!["", "Task", "Dur"])
        .style(Style::new().add_modifier(Modifier::BOLD))
        .bottom_margin(1),
    )
    .block(Block::default()
      .title("Routine")
      .title_alignment(Alignment::Center)
      .borders(Borders::ALL)
      .padding(Padding::new(2, 2, 1, 1))
    )
    .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");
  f.render_stateful_widget(
    table,
    f.size(),
    &mut app.tasks.state
  );
}

fn generate_task_row(task: &app::Task) -> Row {
  Row::new(vec!["[ ]", "Row12", "Row13"])
}
