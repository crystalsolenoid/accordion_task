use ratatui::{
  prelude::{Alignment, Frame},
  style::{Color, Style, Modifier},
  widgets::{List, ListState, ListDirection, Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
  let items = ["Item 1", "Item 2", "Item 3"];
  f.render_stateful_widget(
  List::new(items)
    .block(Block::default().title("Tasks").borders(Borders::ALL))
    .style(Style::default().fg(Color::Yellow))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">> ")
    .repeat_highlight_symbol(true)
    .direction(ListDirection::TopToBottom),
  f.size(), &mut app.tasks.state
  )
}
