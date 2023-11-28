use ratatui::prelude::*;
use std::rc::Rc;

/// Manages layout areas within a TUI application.
///
/// This struct divides a given area into multiple layers and sub-layers, allowing for
/// an organized placement and structuring of UI elements.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Layers {
  /// The topmost layer, typically for headers or top-level controls.
  pub zero: Rc<[Rect]>,
  /// The primary split between two main areas of the UI.
  pub one: Rc<[Rect]>,
  /// The area dedicated to detailed information or secondary content.
  pub details: Rect,
  /// Inner divisions within the details area.
  pub details_inner: Rc<[Rect]>,
  /// Additional layer for auxiliary or less prioritized content.
  pub two: Rc<[Rect]>,
}

impl Layers {
  /// Constructs a new `Layers` instance based on the given area.
  ///
  /// Divides the area into several sub-areas or layers, facilitating organized UI element placement.
  ///
  /// # Arguments
  /// - `area`: The `Rect` representing the entire renderable area.
  ///
  /// # Returns
  /// A new `Layers` instance with defined sub-areas.
  pub fn new(area: Rect) -> Self {
    // Splitting the main area into top, middle, and bottom sections
    let zero = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Max(2),
        Constraint::Min(1),
        Constraint::Max(3),
      ])
      .split(area.inner(&Margin {
        horizontal: 1,
        vertical: 1,
      }));

    // Splitting the middle section into two primary areas
    let one = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![Constraint::Min(40), Constraint::Min(40)])
      .split(zero[1]);

    // Defining the details area
    let details = one[1];

    // Dividing the details area into sub-sections
    let details_inner = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Min(8),
        Constraint::Min(8),
        Constraint::Min(20),
      ])
      .split(details);

    // Creating an additional layer for content
    let two = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![Constraint::Min(1), Constraint::Max(12)])
      .split(one[0]);

    Layers {
      zero,
      one,
      details,
      details_inner,
      two,
    }
  }
}
