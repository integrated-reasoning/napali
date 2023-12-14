use super::buffers::Buffers;
use crate::router::Message;
use crate::tui::Frame;
use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, NoElements, OneElement};
use ratatui::{
  prelude::*,
  widgets::{Axis, Block, Borders, Chart, Dataset},
};
use ringbuffer::RingBuffer;
use tokio::sync::mpsc;

/// Represents an Render FPS (Frames Per Second) chart in Napali.
///
/// # Fields
/// - `message_tx_to_self`: Sender for passing messages to the `RenderFps` component itself.
#[derive(Debug)]
pub struct RenderFps {
  pub message_tx_to_self: mpsc::UnboundedSender<Message>, // cloneable sender to self
}

impl RenderFps {
  /// Creates a chart widget to display FPS data.
  ///
  /// Constructs a chart using the given FPS data, showing the current, minimum,
  /// and maximum FPS values. The chart is styled and configured for optimal visualization.
  ///
  /// # Arguments
  /// - `data`: A vector of tuples containing FPS data, where each tuple consists
  ///   of a time stamp and an FPS value.
  ///
  /// # Returns
  /// A `Chart` widget configured to display the FPS data.
  pub fn chart(data: &Vec<(f64, f64)>) -> Chart<'_> {
    let current_fps = match data.first() {
      Some((_, x)) => *x,
      None => 0.0,
    };
    let (min_fps, max_fps) = match data.iter().map(|(_i, x)| x).minmax() {
      NoElements => (0.0, 0.0),
      MinMax(min, max) => (*min, *max),
      OneElement(x) => (0.0, *x),
    };
    // TODO gate Braille by toggle enhanced graphics
    // TODO add a global toggle for colors
    Chart::new(vec![Dataset::default()
      .name("FPS")
      .marker(symbols::Marker::Braille)
      .style(Style::default().fg(Color::Green))
      .data(data)])
    .block(
      Block::default()
        .title("Render frames per second".bold())
        .borders(Borders::ALL),
    )
    .x_axis(
      Axis::default()
        .style(Style::default().fg(Color::Gray))
        .bounds([0.0, data.len() as f64]),
    )
    .y_axis(
      Axis::default()
        .style(Style::default().fg(Color::Gray))
        .labels(vec![
          if min_fps < current_fps {
            format!("{min_fps:.4}").not_bold()
          } else {
            format!("{min_fps:.4}").bold().light_yellow()
          },
          format!("{current_fps:.4}").bold(),
          if max_fps > current_fps {
            format!("{max_fps:.4}").not_bold()
          } else {
            format!("{max_fps:.4}").bold().light_green()
          },
        ])
        .bounds([0.99 * min_fps, 1.01 * max_fps]),
    )
  }

  fn layer(area: Rect) -> Rect {
    area
  }

  /// Renders the FPS chart onto the specified area of the frame.
  ///
  /// This method takes FPS data from the provided `Buffers` and uses it to create
  /// and render a chart within the given area. The chart visualizes Napali's
  /// render FPS performance over time.
  ///
  /// # Arguments
  /// - `actions`: Reference to the `Buffers` containing FPS data.
  /// - `area`: The area where the chart should be rendered.
  /// - `f`: Mutable reference to the frame for rendering.
  ///
  /// # Returns
  /// `Ok(())` on successful rendering, or an error in case of failure.
  pub fn render(actions: &Buffers, area: Rect, f: &mut Frame<'_>) {
    let layer = Self::layer(area);
    let data = actions
      .render_fps
      .iter()
      .rev()
      .enumerate()
      .map(|(x, &y)| (x as f64, y))
      .collect::<Vec<(f64, f64)>>();
    f.render_widget(Self::chart(&data), layer);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_chart() {
    let data = vec![(0.0, 0.0)];
    let _ = RenderFps::chart(&data);
  }
}
