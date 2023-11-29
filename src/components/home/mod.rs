use super::{Component, State};
use crate::action::{mode::Mode, scene::Scene, view::View, Action};
use crate::router::Message;
use crate::{tui::Event, tui::Frame};
use color_eyre::eyre::Result;
use ratatui::prelude::*;
use tokio::sync::mpsc;
mod about;
mod email_prompt;
mod layers;
use about::About;
use email_prompt::EmailPrompt;

/// Represents the home screen of Napali.
///
/// # Fields
/// - `state`: Current state of the Home component.
/// - `message_tx_to_self`: Sender for passing messages to the Home component itself.
/// - `email_prompt`: Component for handling email prompt functionality.
/// - `about`: Component representing the about section.
/// - `mode`: Current operational mode of the Home component.
#[derive(Debug)]
pub struct Home<'a> {
  state: State,
  pub message_tx_to_self: mpsc::UnboundedSender<Message>,
  email_prompt: EmailPrompt<'a>,
  mode: Mode,
}

impl<'a> Home<'a> {
  /// Constructs a new instance of `Home`.
  ///
  /// Initializes the home component with default values and sets up message channels
  /// for communication with the router and within the component.
  ///
  /// # Arguments
  /// - `tx`: Sender for passing messages to the router.
  ///
  /// # Returns
  /// A new instance of `Home`.
  pub fn new(tx: mpsc::UnboundedSender<Message>) -> Home<'a> {
    let (message_tx_to_self, _) = mpsc::unbounded_channel::<Message>();
    Home {
      state: State::Visible,
      message_tx_to_self,
      email_prompt: EmailPrompt::new(tx),
      mode: Mode::default(),
    }
  }

  /// Determines if Napali should return to navigation mode from text input mode.
  ///
  /// # Returns
  /// `true` if the email prompt is inactive and the current mode is `TextInput`, otherwise `false`.
  fn should_restore_navigation_mode(&self) -> bool {
    !self.email_prompt.is_active() && (self.mode == Mode::TextInput)
  }
}

impl<'a> Component for Home<'a> {
  /// Updates the state and mode of the Home component based on the given action.
  ///
  /// Handles the transition between scenes and views, and manages activation of the email prompt.
  ///
  /// # Arguments
  /// - `action`: The action to process.
  ///
  /// # Returns
  /// A result indicating successful processing and optionally a new action to be taken.
  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    // Handle mode change actions directly
    if let Action::ChangeMode(mode) = action {
      self.mode = mode;
    }

    // Process actions based on the current mode
    match self.mode {
      Mode::Navigation => match action {
        Action::ChangeScene(scene) => {
          // Handle scene change actions
          match scene {
            Scene::Home => self.state = State::Visible,
            _ => self.state = State::Hidden,
          }
        }
        Action::ChangeView(k) => {
          // Activate the email prompt if the view is prompt and home is visible
          if self.state == State::Visible {
            if let View::Prompt = k {
              return self.email_prompt.activate();
            }
          }
        }
        _ => {}
      },
      Mode::TextInput => {
        // Restore navigation mode if applicable
        if self.should_restore_navigation_mode() {
          return Ok(Some(Action::ChangeMode(Mode::Navigation)));
        }
      }
    }
    Ok(None)
  }

  /// Handles external events like key presses affecting this component.
  ///
  /// # Arguments
  /// - `event`: The event to process.
  ///
  /// # Returns
  /// A result indicating successful processing and optionally a new action to be taken.
  fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>> {
    // Process key events if the email prompt is active
    if self.email_prompt.is_active() {
      if let Some(Event::Key(k)) = event {
        self.email_prompt.handle_key_event(k)?;
      }
    }
    Ok(None)
  }

  /// Renders the Home component onto the terminal frame.
  ///
  /// This method is responsible for drawing the home screen, including the email prompt and about section.
  ///
  /// # Arguments
  /// - `f`: Mutable reference to the terminal frame.
  /// - `area`: The area of the terminal to draw in.
  ///
  /// # Returns
  /// `Ok(())` on successful rendering, or an error in case of failure.
  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    // Only render if the state is not hidden
    if let State::Hidden = self.state {
    } else {
      // Use a layout manager for structuring the UI
      let layers = layers::Layers::new(area);
      // Render the email prompt and about section
      self.email_prompt.render(layers.zero[2], f);
      About::render(layers.zero[1], f);
    }
    Ok(())
  }
}
