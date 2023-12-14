use super::{Component, State};
use crate::action::mode::Mode;
use crate::action::{scene::Scene, Action};
use crate::router::Message;
use crate::tui::Frame;
use color_eyre::eyre::Result;
use ratatui::prelude::*;
use tokio::sync::mpsc;
mod jobs;
mod layers;
mod prompt;
mod widgets;
use jobs::Jobs;
use prompt::Prompt;

/// Manages the session interface in a TUI application.
///
/// This struct handles various components of a session, including prompts, jobs,
/// workspaces, and other widgets. It manages their states and renders them accordingly.
#[derive(Debug)]
pub struct Session<'a> {
  state: State,
  pub message_tx_to_self: mpsc::UnboundedSender<Message>,
  prompt: Prompt<'a>,
  jobs: Jobs<'a>,
  workspaces: widgets::Workspaces<'a>,
  status: widgets::Status<'a>,
  plots: widgets::Plots<'a>,
  logs: widgets::Logs<'a>,
  mode: Mode,
}

impl<'a> Session<'a> {
  /// Constructs a new `Session`.
  ///
  /// Initializes the session with default components and state.
  pub fn new() -> Session<'a> {
    let (message_tx_to_self, _) = mpsc::unbounded_channel::<Message>();
    Session {
      state: State::Hidden,
      message_tx_to_self,
      prompt: Prompt::new(),
      jobs: Jobs::new(),
      workspaces: widgets::Workspaces::new(),
      status: widgets::Status::new(),
      plots: widgets::Plots::new(),
      logs: widgets::Logs::new(),
      mode: Mode::default(),
    }
  }
}

impl<'a> Component for Session<'a> {
  /// Updates the session based on the given action.
  ///
  /// Handles mode changes and view updates, managing the visibility and state of session components.
  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    if let Action::ChangeMode(mode) = action {
      self.mode = mode;
    }
    if self.mode == Mode::Navigation {
      match action {
        Action::ChangeScene(scene) => match scene {
          Scene::Session => self.state = State::Visible,
          _ => self.state = State::Hidden,
        },
        Action::ChangeView(k) => {
          if self.state == State::Visible {
            if let jobs::View::Prompt = jobs::View::from(k) {
              self.prompt.toggle();
            } else {
              self.jobs.set_view(k);
            }
          }
        }
        _ => {}
      }
    }
    Ok(None)
  }

  /// Draws the session components onto the terminal frame.
  ///
  /// Renders each component in its designated area, based on the current state and mode.
  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    if self.state == State::Hidden {
      Ok(())
    } else {
      let layers = layers::Layers::new(area);
      self.jobs.render(layers.two[0], f);
      f.render_widget(self.workspaces.block.clone(), layers.two[1]);
      self.prompt.render(layers.zero[2], f);
      f.render_widget(self.status.block.clone(), layers.details_inner[0]);
      f.render_widget(self.plots.block.clone(), layers.details_inner[1]);
      f.render_widget(self.logs.block.clone(), layers.details_inner[2]);
      Ok(())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_session_new() {
    let _ = Session::new();
  }
}
