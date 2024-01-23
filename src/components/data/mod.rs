use super::{Component, State};
use crate::action::mode::Mode;
use crate::action::{scene::Scene, Action};
use crate::router::Message;
use crate::tui::Frame;
use color_eyre::eyre::Result;
use ratatui::prelude::*;
use tokio::sync::mpsc;
mod layers;
mod widgets;

/// Manages the data interface.
///
/// This struct handles various components of the data interface.
#[derive(Debug)]
pub struct Data {
  state: State,
  pub message_tx_to_self: mpsc::UnboundedSender<Message>,
  stats: widgets::Stats,
  mode: Mode,
}

impl Data {
  /// Constructs a new `Data`.
  ///
  /// Initializes the interface with default components and state.
  pub fn new() -> Data {
    let (message_tx_to_self, _) = mpsc::unbounded_channel::<Message>();
    Data {
      state: State::Hidden,
      message_tx_to_self,
      stats: widgets::Stats::new(),
      mode: Mode::default(),
    }
  }
}

impl Component for Data {
  /// Updates the data interface based on the given action.
  ///
  /// Handles mode changes and view updates, managing the visibility and state of components.
  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    if let Action::ChangeMode(mode) = action {
      self.mode = mode;
    }
    if self.mode == Mode::Navigation {
      match action {
        Action::ChangeScene(scene) => match scene {
          Scene::Data => self.state = State::Visible,
          _ => self.state = State::Hidden,
        },
        Action::ChangeView(k) => {
          if self.state == State::Visible {
            //
          }
        }
        _ => {}
      }
    }
    Ok(None)
  }

  /// Draws the components onto the terminal frame.
  ///
  /// Renders each component in its designated area, based on the current state and mode.
  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    if self.state == State::Hidden {
      Ok(())
    } else {
      let layers = layers::Layers::new(area);
      self.stats.render(layers.one[0], f);
      Ok(())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_data_new() {
    let _ = Data::new();
  }
}
