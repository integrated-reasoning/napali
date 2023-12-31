use super::buffers::Buffers;
use crate::router::Message;
use crate::tui::Frame;
use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, NoElements, OneElement};
use ratatui::{
  prelude::*,
  widgets::{block::Block, Axis, Borders, Chart, Dataset},
};
use ringbuffer::RingBuffer;
use tokio::sync::mpsc;

/// Represents an App FPS (Frames Per Second) chart in Napali.
///
/// # Fields
/// - `message_tx_to_self`: Sender for passing messages to the `AppFps` component itself.
#[derive(Debug)]
pub struct AppFps {
  pub message_tx_to_self: mpsc::UnboundedSender<Message>,
}

impl AppFps {
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
  fn chart(data: &Vec<(f64, f64)>) -> Chart<'_> {
    // Determine the current, minimum, and maximum FPS values
    let current_fps = data.first().map_or(0.0, |(_i, x)| *x);
    let (min_fps, max_fps) = match data.iter().map(|(_i, x)| x).minmax() {
      NoElements => (0.0, 0.0),
      MinMax(min, max) => (*min, *max),
      OneElement(x) => (*x, *x),
    };

    // Create a Chart widget with the FPS data
    Chart::new(vec![Dataset::default()
      .name("FPS")
      .marker(symbols::Marker::Braille)
      .style(Style::default().fg(Color::Green))
      .data(data)])
    .block(
      Block::default()
        .title("App ticks per second".bold())
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
            format!("{min_fps:.8}").not_bold()
          } else {
            format!("{min_fps:.8}").bold().light_yellow()
          },
          format!("{current_fps:.8}").bold(),
          if max_fps > current_fps {
            format!("{max_fps:.8}").not_bold()
          } else {
            format!("{max_fps:.8}").bold().light_green()
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
  /// app FPS performance over time.
  ///
  /// # Arguments
  /// - `actions`: Reference to the `Buffers` containing FPS data.
  /// - `area`: The area where the chart should be rendered.
  /// - `f`: Mutable reference to the frame for rendering.
  ///
  /// # Returns
  /// `Ok(())` on successful rendering, or an error in case of failure.
  pub fn render(actions: &Buffers, area: Rect, f: &mut Frame<'_>) {
    // Determine the layout area for the chart
    let layer = Self::layer(area);

    // Prepare the data for the chart
    // The data is collected in reverse to show the most recent FPS values
    let data = actions
      .app_fps
      .iter()
      .rev()
      .enumerate()
      .map(|(x, &y)| (x as f64, y))
      .collect::<Vec<(f64, f64)>>();

    // Create and render the FPS chart with the prepared data
    f.render_widget(Self::chart(&data), layer);
  }
}
