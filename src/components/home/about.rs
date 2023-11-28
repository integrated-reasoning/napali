use crate::tui::Frame;
use ratatui::{
  prelude::*,
  widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

/// Represents an 'About' section in the TUI application.
///
/// This struct is responsible for rendering informational text about the application,
/// including user credits and terms.
#[derive(Debug)]
pub struct About {}

impl About {
  /// Creates a paragraph widget containing the about section text.
  ///
  /// # Returns
  /// A `Paragraph` widget configured with the about section content.
  fn about() -> Paragraph<'static> {
    let text = vec![
        Line::from(""),
        Line::from("Early Users"),
        Line::from(""),
        Line::from("Thank you early adopters! This iteration of Napali provides the first render test of our optimization service."),
        Line::from(""),
        Line::from("Upon running Napali, an API key is provisioned and written to ~/.config/irx"),
        Line::from(""),
        Line::from("Please take a moment to look around. Your feedback is sincerely appreciated and will help us better serve the optimization community. Suggestions and findings, especially screenshots of how Napali renders in your terminal may be sent to hello@integrated-reasoning.com."),
        Line::from("Additionally as a special thank you, registering your email using Napali before December 31st, 2023 will grant you early adopter status."),
        Line::from(""),
        Line::from("In the coming weeks, early adopters will be the first to receive details on upcoming features."),
        Line::from(""),
        Line::from("Terms:"),
        Line::from("THE LICENSED SOFTWARE IS PROVIDED \"AS IS\" AND, TO THE MAXIMUM EXTENT PERMITTED UNDER APPLICABLE LAW, INTEGRATED REASONING, INC. MAKES NO WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, BY STATUTE OR OTHERWISE, INCLUDING BUT NOT LIMITED TO ANY IMPLIED WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, TITLE, OR NON-INFRINGMENT."),
    ];
    Paragraph::new(text)
      .block(
        Block::default()
          .title("Developer Preview")
          .title_alignment(Alignment::Left)
          .borders(Borders::ALL)
          .border_style(Style::default())
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .wrap(Wrap { trim: false })
      .alignment(Alignment::Left)
  }

  /// Determines the layout area for the about section.
  ///
  /// # Arguments
  /// - `area`: The `Rect` representing the entire renderable area.
  ///
  /// # Returns
  /// A `Rect` defining the area for the about section.
  fn layer(area: Rect) -> Rect {
    area
  }

  /// Renders the about section onto the specified area of the frame.
  ///
  /// # Arguments
  /// - `area`: The area where the about section should be rendered.
  /// - `f`: Mutable reference to the frame for rendering.
  pub fn render(area: Rect, f: &mut Frame<'_>) {
    let layer = Self::layer(area);
    let text = Self::about();
    f.render_widget(text, layer);
  }
}
