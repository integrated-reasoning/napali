use ratatui::prelude::*;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Layers {
  pub zero: Rc<[Rect]>,
  pub one: Rc<[Rect]>,
}

impl Layers {
  pub fn new(area: Rect) -> Self {
    let zero = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![Constraint::Max(2), Constraint::Min(1)])
      .split(area.inner(&Margin {
        horizontal: 1,
        vertical: 1,
      }));

    let one = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![Constraint::Min(1)])
      .split(zero[1]);

    Layers { zero, one }
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
