use ratatui::prelude::*;
use std::rc::Rc;

/// Manages layout areas within Napali.
///
/// This struct is responsible for dividing a given area into multiple sub-areas (layers),
/// enabling an organized and flexible placement of UI elements within the Internals component.
/// The layout is managed using the `ratatui` crate's layout system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Layers {
  /// The topmost layer, typically used for headers or top-level controls.
  pub zero: Rc<[Rect]>,
  /// The main layer, usually split into left and right sub-areas.
  pub one: Rc<[Rect]>,
  /// The left sub-area of the main layer, often used for navigation or lists.
  pub left: Rc<[Rect]>,
  /// The right sub-area of the main layer, commonly used for content display.
  pub right: Rc<[Rect]>,
}

impl Layers {
  /// Constructs a new `Layers` instance based on the given area.
  ///
  /// Divides the area into several sub-areas using a combination of horizontal and vertical splits.
  ///
  /// # Arguments
  /// - `area`: The `Rect` representing the entire renderable area.
  ///
  /// # Returns
  /// A new `Layers` instance with defined sub-areas.
  pub fn new(area: Rect) -> Self {
    // Top layer split into two parts
    let zero = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![Constraint::Max(2), Constraint::Min(1)])
      .split(area.inner(&Margin {
        horizontal: 1,
        vertical: 1,
      }));

    // Main layer split into left and right sections
    let one = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
      .split(zero[1]);

    // Left section split into upper and lower parts
    let left = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![Constraint::Min(10), Constraint::Min(0)])
      .split(one[0]);

    // Right section split into three parts
    let right = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Min(5),
        Constraint::Min(5),
        Constraint::Min(0),
      ])
      .split(one[1]);

    Layers {
      zero,
      one,
      left,
      right,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_layers_new() {
    let rect = Rect::default();
    let _ = Layers::new(rect);
  }
}
