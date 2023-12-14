use super::Component;
use crate::action::{mode::Mode, scene::Scene, Action};
use crate::tui::Frame;
use color_eyre::eyre::Result;
use ratatui::{
  prelude::*,
  widgets::{Block, Borders, Paragraph, Tabs},
};

/// The base layer and tab bar of a TUI application.
///
/// # Fields
/// - `message_tx_to_self`: Sender for passing messages to the Base component itself.
/// - `scene`: Current UI scene being displayed.
/// - `mode`: Current operational mode of the Base component.
#[derive(Debug)]
pub struct Base {
  scene: Scene,
  mode: Mode,
}

impl Base {
  /// Constructs a new instance of `Base`.
  ///
  /// Initializes the component with default values and sets up message channels
  /// for communication with the router and within the component.
  ///
  /// # Returns
  /// A new instance of `Base`.
  pub fn new() -> Base {
    Base {
      scene: Scene::default(),
      mode: Mode::default(),
    }
  }
}

impl Component for Base {
  /// Updates the component's state based on the given action.
  ///
  /// Changes the mode of the component if the action is `ChangeMode`.
  /// In `Navigation` mode, it also handles `ChangeScene` actions.
  ///
  /// # Arguments
  /// - `action`: The action to process.
  ///
  /// # Returns
  /// `Ok(None)` indicating successful processing without new actions,
  /// or an error in case of failure.
  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    if let Action::ChangeMode(mode) = action {
      self.mode = mode;
    }
    if self.mode == Mode::Navigation {
      if let Action::ChangeScene(scene) = action {
        self.scene = scene;
      }
    }
    Ok(None)
  }

  /// Renders the component onto the terminal frame.
  ///
  /// This method is responsible for drawing the base layer and the tab bar
  /// of the UI. It sets up the layout and positions UI elements like tabs
  /// and blocks accordingly.
  ///
  /// # Arguments
  /// - `f`: Mutable reference to the terminal frame.
  /// - `area`: The area of the terminal to draw in.
  ///
  /// # Returns
  /// `Ok(())` on successful rendering, or an error in case of failure.
  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let layer_zero = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Max(2), Constraint::Min(1), Constraint::Max(3)])
      .split(area.inner(&Margin {
        horizontal: 1,
        vertical: 1,
      }));

    let layer_top_bar = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Max(3)])
      .split(layer_zero[0]);

    let layer_top_bar_left_right = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([Constraint::Max(40), Constraint::Min(14)])
      .split(layer_top_bar[0]);

    f.render_widget(
      Block::default().borders(Borders::BOTTOM),
      layer_top_bar[0],
    );

    let top_tabs = Tabs::new(
      ["Home", "Session", "Internals", "? Usage"]
        .iter()
        .map(|t| {
          let (first, rest) = t.split_at(1);
          Line::from(vec![first.white(), rest.gray()])
        })
        .collect::<Vec<_>>(),
    )
    .block(Block::default().borders(Borders::NONE))
    .select(match self.scene {
      Scene::Home => 0,
      Scene::Session => 1,
      Scene::Internals => 2,
    })
    .style(Style::default())
    .highlight_style(Style::default().bold());

    f.render_widget(top_tabs, layer_top_bar_left_right[0]);

    // Render version information on the right side of the top bar
    f.render_widget(
      Paragraph::new("Napali v0.1.1")
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Right),
      layer_top_bar_left_right[1],
    );

    // Render the main frame border
    f.render_widget(
      Block::default()
        .title("Integrated Reasoning, Inc.")
        .title_alignment(Alignment::Right)
        .borders(Borders::ALL)
        .border_style(Style::default()),
      area,
    );

    Ok(())
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_base_new() {
    let _ = Base::new();
  }
}
