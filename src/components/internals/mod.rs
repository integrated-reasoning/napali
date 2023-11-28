use super::{Component, State};
use crate::action::{mode::Mode, scene::Scene, Action};
use crate::router::Message;
use crate::tui::Frame;
use color_eyre::eyre::Result;
use ratatui::prelude::*;
use tokio::sync::mpsc;
mod app_fps;
mod buffers;
mod counters;
mod layers;
mod render_fps;
mod state_display;
mod trail;
use state_display::StateDisplay;
mod stats_display;
use app_fps::AppFps;
use buffers::Buffers;
use counters::Counters;
use render_fps::RenderFps;
use stats_display::StatsDisplay;
use trail::Trail;

/// Represents the internals and diagnostic information for Napali.
///
/// # Fields
/// - `state`: Current state of the Internals component.
/// - `message_tx_to_self`: Sender for passing messages to the Internals component itself.
/// - `state_display`: Component for displaying the state.
/// - `stats_display`: Component for displaying statistics.
/// - `actions`: Buffer storing actions.
/// - `counters`: Counters for various metrics.
/// - `mode`: Current operational mode of the Internals component.
#[derive(Debug)]
pub struct Internals {
  state: State,
  pub message_tx_to_self: mpsc::UnboundedSender<Message>,
  state_display: StateDisplay,
  stats_display: StatsDisplay,
  actions: Buffers,
  counters: Counters,
  mode: Mode,
}

impl Internals {
  /// Constructs a new instance of `Internals`.
  ///
  /// Initializes the internals component with default values and sets up message channels
  /// for communication with the router and within the component.
  ///
  /// # Arguments
  /// - `tx`: Sender for passing messages to the router.
  ///
  /// # Returns
  /// A new instance of `Internals`.
  pub fn new(tx: mpsc::UnboundedSender<Message>) -> Internals {
    let (message_tx_to_self, _) = mpsc::unbounded_channel::<Message>();
    Internals {
      state: State::Visible,
      message_tx_to_self,
      state_display: StateDisplay::new(tx),
      stats_display: StatsDisplay::new(),
      actions: Buffers::default(),
      counters: Counters::default(),
      mode: Mode::default(),
    }
  }

  /// Retrieves the message transmission handle for the state display.
  ///
  /// # Returns
  /// An `UnboundedSender<Message>` for the state display.
  pub fn get_state_display_tx_handle(&self) -> mpsc::UnboundedSender<Message> {
    self.state_display.get_tx_handle()
  }
}

impl Component for Internals {
  /// Updates the state of the Internals component based on the received action.
  ///
  /// Processes actions related to mode changes, ticks, rendering, and scene changes.
  ///
  /// # Arguments
  /// - `action`: The action that triggers the state update.
  ///
  /// # Returns
  /// `Ok(None)`: Indicating the action was processed without generating a new action.
  /// `Err(_)`: If any error occurs during processing.
  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    // Handle mode changes directly
    if let Action::ChangeMode(mode) = action {
      self.mode = mode;
    }

    // Process specific actions based on the current mode
    match action {
      Action::Tick => {
        // Process application ticks
        self.stats_display.app_tick(&mut self.counters);
      }
      Action::Render => {
        // Process rendering ticks
        self.stats_display.render_tick(&mut self.counters);
      }
      Action::ChangeScene(scene) => match self.mode {
        Mode::Navigation => {
          // Update visibility based on scene changes
          match scene {
            Scene::Internals => self.state = State::Visible,
            _ => self.state = State::Hidden,
          }
        }
        Mode::TextInput => {}
      },
      _ => {}
    }

    // Update stats display with the current action and counters
    StatsDisplay::update(&mut self.actions, &mut self.counters, action);
    Ok(None)
  }

  /// Renders the Internals component onto the terminal frame.
  ///
  /// This method is responsible for drawing the internal state, statistics, FPS plots, and trails.
  ///
  /// # Arguments
  /// - `f`: Mutable reference to the terminal frame where UI elements are rendered.
  /// - `area`: The area of the terminal where the Internals component is drawn.
  ///
  /// # Returns
  /// `Ok(())`: On successful rendering.
  /// `Err(_)`: If any error occurs during rendering.
  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    // Render only if the state is not hidden
    if let State::Hidden = self.state {
    } else {
      // Arrange UI elements using a layout manager
      let layers = layers::Layers::new(area);

      // Render individual components
      self.state_display.render(layers.left[0], f)?;
      RenderFps::render(&self.actions, layers.right[0], f);
      AppFps::render(&self.actions, layers.right[1], f);
      StatsDisplay::render(&self.counters, layers.left[1], f)?;

      // Render the trail component
      Trail::render(&self.actions, layers.right[2], f);
    }
    Ok(())
  }
}
