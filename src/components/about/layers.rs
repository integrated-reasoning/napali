use ratatui::prelude::*;
use std::rc::Rc;

/// Represents the layers within a graphical user interface.
///
/// This struct organizes different layers of UI components, each represented by a `Rect`.
/// The use of `Rc<[Rect]>` implies that these layers are reference-counted and can be shared
/// across multiple parts of the UI without full ownership, promoting efficient memory usage.
///
/// Fields:
/// - `zero`: The bottom layer, typically used for background or base elements.
/// - `one`: A layer above `zero`, often used for primary UI elements.
/// - `details`: A specific `Rect` for detailed or focused content.
/// - `details_inner`: Layers within the `details` area, allowing for nested or complex UI structures.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Layers {
  pub zero: Rc<[Rect]>,
  pub one: Rc<[Rect]>,
  pub details: Rect,
  pub details_inner: Rc<[Rect]>,
}

impl Layers {
  /// Creates a new `Layers` instance based on the provided `Rect`.
  ///
  /// This method initializes the different layers of the UI, positioning them according to
  /// the given area and specific layout constraints. It demonstrates how various UI elements
  /// are organized in a hierarchical and structured manner.
  ///
  /// Parameters:
  /// - `area`: The `Rect` defining the overall area available for the layers.
  ///
  /// Returns:
  /// A new instance of `Layers` with initialized fields.
  pub fn new(area: Rect) -> Self {
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

    let one = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![Constraint::Min(40), Constraint::Min(40)])
      .split(zero[1]);
    let details = one[1];
    let details_inner = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Min(8),
        Constraint::Min(8),
        Constraint::Min(20),
      ])
      .split(details);

    Layers {
      zero,
      one,
      details,
      details_inner,
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
