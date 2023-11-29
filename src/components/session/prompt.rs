use ratatui::{
  prelude::*,
  widgets::{Block, Borders},
};
use tui_textarea::TextArea;

/// Manages a prompt for text input in a TUI application.
///
/// This struct handles the display and state of a text area where users can input text.
/// It manages the active state and styling of the text area.
#[derive(Debug)]
pub struct Prompt<'a> {
  text: TextArea<'a>,
  is_active: bool,
}

impl<'a> Prompt<'a> {
  /// Constructs a new `Prompt` with default settings.
  pub fn new() -> Self {
    Prompt {
      text: TextArea::default(),
      is_active: false,
    }
  }

  /// Calculates the layout area for the prompt based on its active state.
  ///
  /// # Arguments
  /// - `area`: The `Rect` representing the entire renderable area.
  ///
  /// # Returns
  /// A `Rect` defining the area for the prompt.
  fn layer(&self, area: Rect) -> Rect {
    (if self.is_active {
      Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(3)])
        .split(area.inner(&Margin {
          horizontal: 1,
          vertical: 0,
        }))
    } else {
      Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(3)])
        .split(area)
    })[0]
  }

  /// Sets the style of the text area based on whether the prompt is active.
  fn set_style(&mut self) {
    if self.is_active {
      self.text.set_cursor_line_style(Style::default());
      self.text.set_placeholder_text(">");
      self.text.set_block(
        Block::default()
          .borders(Borders::ALL)
          .style(Style::default().fg(Color::LightGreen)),
      );
    } else {
      self.text.set_cursor_line_style(Style::default());
      self.text.set_placeholder_text(">");
      //let mut is_valid = validate(&mut self.text);
      self.text.set_block(
        Block::default()
          .title("Prompt")
          .title_alignment(Alignment::Left)
          .borders(Borders::ALL)
          .style(Style::default()),
      );
    }
  }

  /// Toggles the active state of the prompt.
  pub fn toggle(&mut self) {
    self.is_active = !self.is_active;
  }

  /// Renders the prompt onto the specified area of the frame.
  ///
  /// # Arguments
  /// - `area`: The area where the prompt should be rendered.
  /// - `f`: Mutable reference to the frame for rendering.
  pub fn render(&mut self, area: Rect, f: &mut Frame<'_>) {
    let layer = self.layer(area);
    self.set_style();
    f.render_widget(self.text.widget(), layer);
  }
}
