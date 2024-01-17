use ratatui::{
  prelude::{Alignment, Frame},
  style::{Color, Style, Modifier},
  widgets::{List, ListState, ListDirection, Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
  /**
  f.render_widget(
  Paragraph::new(format!(
    "
      Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
      Press `j` and `k` to increment and decrement the counter respectively.\n\
      Counter: {}
    ",
    app.counter
  ))
  .block(
    Block::default()
    .title("Counter App")
    .title_alignment(Alignment::Center)
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded),
  )
  .style(Style::default().fg(Color::Yellow))
  .alignment(Alignment::Center),
  f.size(),
  );
  **/
  let items = ["Item 1", "Item 2", "Item 3"];
  f.render_stateful_widget(
  List::new(items)
    .block(Block::default().title("Tasks").borders(Borders::ALL))
    .style(Style::default().fg(Color::Yellow))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">> ")
    .repeat_highlight_symbol(true)
    .direction(ListDirection::TopToBottom),
  f.size(), &mut app.current_task
  )
}
