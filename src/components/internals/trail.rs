use super::buffers::Buffers;
use crate::router::Message;
use crate::tui::Frame;
use ratatui::{
  prelude::*,
  widgets::{block::Block, Borders, List, ListItem},
};
use ringbuffer::RingBuffer;
use tokio::sync::mpsc;

/// Represents a trail of actions taken in the application.
///
/// This struct is responsible for rendering a list of actions (or 'trail') in a TUI environment,
/// displaying the historical sequence of actions performed by the user or the application.
#[derive(Debug)]
pub struct Trail {
  /// Sender for passing messages to the Trail component itself.
  pub message_tx_to_self: mpsc::UnboundedSender<Message>,
}

impl Trail {
  /// Determines the layout area for rendering the trail display.
  ///
  /// # Arguments
  /// - `area`: The `Rect` representing the entire renderable area.
  ///
  /// # Returns
  /// A `Rect` defining the area for the trail display.
  fn layer(area: Rect) -> Rect {
    area
  }

  /// Renders the trail of actions onto the specified area of the frame.
  ///
  /// Constructs and renders a list widget containing the sequence of actions.
  ///
  /// # Arguments
  /// - `actions`: Reference to the `Buffers` containing the trail data.
  /// - `area`: The area where the trail should be rendered.
  /// - `f`: Mutable reference to the frame for rendering.
  pub fn render(actions: &Buffers, area: Rect, f: &mut Frame<'_>) {
    let layer = Self::layer(area);

    // Constructing a list of actions from the trail buffer
    let items = actions
      .trail
      .iter()
      .map(|s| ListItem::new(s.clone()))
      .collect::<Vec<ListItem>>();

    // Creating and configuring the List widget for trail display
    let trail =
      List::new(items).block(Block::new().borders(Borders::ALL).title("Trail"));

    // Rendering the trail list in the specified area
    f.render_widget(trail, layer);
  }
}
