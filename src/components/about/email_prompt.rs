use crate::action::mode::Mode;
use crate::action::Action;
use crate::router::{Address, Cacheable, Kind, Message, Payload};
use crate::tui::Frame;
use color_eyre::eyre::{eyre, Result};
use crossterm::event::KeyEvent;
use email_address::EmailAddress;
use ratatui::{
  prelude::*,
  widgets::{block::Block, Borders},
};
use tokio::sync::mpsc;
use tui_textarea::{Input, Key, TextArea};

/// A component for prompting and validating an email address input.
///
/// This struct manages the display and interaction of a text area where users can input an email address.
/// It provides functionality to activate, deactivate, validate, and handle key events related to the email input.
///
/// # Fields
/// - `message_tx_to_router`: Sender for passing messages to the router.
/// - `text`: `TextArea` widget for email input.
/// - `is_active`: Boolean indicating if the prompt is currently active.
/// - `is_valid`: Boolean indicating if the entered email is valid.
#[derive(Debug)]
pub struct EmailPrompt<'a> {
  message_tx_to_router: mpsc::UnboundedSender<Message>,
  text: TextArea<'a>,
  is_active: bool,
  is_valid: bool,
}

impl<'a> EmailPrompt<'a> {
  /// Constructs a new instance of `EmailPrompt`.
  ///
  /// Initializes the email prompt with a message sender and default states.
  ///
  /// # Arguments
  /// - `tx`: Sender for passing messages to the router.
  ///
  /// # Returns
  /// A new instance of `EmailPrompt`.
  pub fn new(tx: mpsc::UnboundedSender<Message>) -> EmailPrompt<'a> {
    EmailPrompt {
      message_tx_to_router: tx,
      text: TextArea::default(),
      is_active: false,
      is_valid: false,
    }
  }

  /// Configures the layout and style of the email prompt based on its active state.
  ///
  /// # Arguments
  /// - `area`: The area where the prompt should be rendered.
  ///
  /// # Returns
  /// The `Rect` defining the area for the text field.
  fn layer(&mut self, area: Rect) -> Rect {
    self.configure_text_field();

    let layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Max(3)]);

    if self.is_active {
      self.text.set_block(Self::active_block_style());
      layout.split(area.inner(&Margin {
        horizontal: 1,
        vertical: 0,
      }))[0]
    } else {
      self.text.set_block(Self::inactive_block_style());
      layout.split(area)[0]
    }
  }

  /// Sets common properties for the text field.
  fn configure_text_field(&mut self) {
    self.text.set_cursor_line_style(Style::default());
    self.text.set_placeholder_text("Your email");
    self.text.set_placeholder_style(Style::default());
  }

  /// Creates a block style for the active state of the prompt.
  fn active_block_style() -> Block<'a> {
    Block::default()
      .borders(Borders::ALL)
      .style(Style::default().fg(Color::LightGreen))
  }

  /// Creates a block style for the inactive state of the prompt.
  fn inactive_block_style() -> Block<'a> {
    Block::default()
      .title("Email Prompt")
      .title_alignment(Alignment::Left)
      .borders(Borders::ALL)
      .style(Style::default())
  }

  /// Activates the email prompt.
  ///
  /// # Returns
  /// A result indicating success or failure, and optionally an action to be taken.
  pub fn activate(&mut self) -> Result<Option<Action>> {
    self.is_active = true;
    Ok(Some(Action::ChangeMode(Mode::TextInput)))
  }

  /// Deactivates the email prompt.
  pub fn deactivate(&mut self) {
    self.is_active = false;
  }

  /// Checks if the email prompt is currently active.
  ///
  /// # Returns
  /// `true` if active, otherwise `false`.
  pub fn is_active(&self) -> bool {
    self.is_active
  }

  /// Sends a message to upgrade the API key based on the provided email address.
  ///
  /// # Arguments
  /// - `email`: The email address used for the API key upgrade.
  ///
  /// # Returns
  /// A result indicating success or failure of the operation.
  fn upgrade_api_key(&mut self, email: EmailAddress) -> Result<()> {
    self
      .message_tx_to_router
      .send(Message {
        source: Address::About,
        destination: Address::IrxClient,
        payload: Payload::Email(email),
        tag: None,
        cacheable: Cacheable::No,
        kind: Kind::Tell,
      })
      .map_err(|e| eyre!(e))
  }

  /// Renders the email prompt in the given area.
  ///
  /// # Arguments
  /// - `area`: The area where the prompt should be rendered.
  /// - `f`: The frame used for rendering.
  ///
  /// # Returns
  /// A result indicating success or failure of rendering.
  pub fn render(&mut self, area: Rect, f: &mut Frame<'_>) {
    let layer = self.layer(area);
    f.render_widget(self.text.widget(), layer);
  }

  /// Handles key events for the email prompt.
  ///
  /// Processes various key inputs, managing the prompt's state and actions based on the input.
  ///
  /// # Arguments
  /// - `key_event`: The `KeyEvent` to be handled.
  ///
  /// # Returns
  /// A result indicating success or failure of handling the key event.
  pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
    self.validate();
    match key_event.into() {
      Input { key: Key::Esc, .. }
      | Input {
        key: Key::Char('c'),
        ctrl: true,
        ..
      } => {
        self.deactivate();
      }
      Input {
        key: Key::Enter, ..
      } if self.is_valid => {
        self.deactivate();
        let email = self.get_email()?;
        if self.upgrade_api_key(email).is_ok() {
          // TODO: indicate success and disable the email prompt for the session
          self.reset();
        }
      }
      input => {
        if self.text.lines()[0].len() < 80 {
          if self.text.input(input) {
            self.validate();
          }
        } else {
          self.reset();
        }
      }
    }
    Ok(())
  }

  /// Resets the text field and validation states of the email prompt.
  fn reset(&mut self) {
    self.text = TextArea::default();
    self.is_valid = false;
    self.deactivate();
  }

  /// Validates the current input in the text field as an email address.
  ///
  /// Updates the style of the text field based on the validity of the input.
  ///
  /// # Returns
  /// `true` if the input is a valid email address, otherwise `false`.
  fn validate(&mut self) -> bool {
    if let Err(_err) = self.text.lines()[0].parse::<EmailAddress>() {
      self.text.set_style(Style::default().fg(Color::Yellow));
      self.text.set_block(
        Block::default()
          .borders(Borders::ALL)
          .title("Press Esc to cancel"),
      );
      self.is_valid = false;
    } else {
      self.text.set_style(Style::default().fg(Color::LightGreen));
      self.text.set_block(
        Block::default()
          .borders(Borders::ALL)
          .title("Press Enter to submit"),
      );
      self.is_valid = true;
    }
    self.is_valid
  }

  /// Retrieves the current email input as an `EmailAddress`.
  ///
  /// # Returns
  /// A result containing the `EmailAddress` if valid, or an error otherwise.
  pub fn get_email(&mut self) -> Result<EmailAddress> {
    self.text.lines()[0]
      .parse::<EmailAddress>()
      .map_err(|e| eyre!(e))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use color_eyre::Result;
  use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

  #[test]
  fn test_email_prompt_new() {
    let (tx, _) = mpsc::unbounded_channel::<Message>();
    let _ = EmailPrompt::new(tx);
  }

  #[test]
  fn test_layer() {
    let (tx, _) = mpsc::unbounded_channel::<Message>();
    let mut prompt = EmailPrompt::new(tx);
    let _ = prompt.layer(Rect::default());
  }

  #[test]
  fn test_reset() {
    let (tx, _) = mpsc::unbounded_channel::<Message>();
    let mut prompt = EmailPrompt::new(tx);
    prompt.reset();
  }

  #[test]
  fn test_activate() -> Result<()> {
    let (tx, _) = mpsc::unbounded_channel::<Message>();
    let mut prompt = EmailPrompt::new(tx);
    let _ = prompt.activate()?;
    Ok(())
  }

  #[test]
  fn test_deactivate() {
    let (tx, _) = mpsc::unbounded_channel::<Message>();
    let mut prompt = EmailPrompt::new(tx);
    prompt.deactivate();
  }

  #[test]
  fn test_is_active() {
    let (tx, _) = mpsc::unbounded_channel::<Message>();
    let mut prompt = EmailPrompt::new(tx);
    let _ = prompt.activate();
    assert!(prompt.is_active());
    prompt.deactivate();
    assert!(!prompt.is_active());
    let _ = prompt.activate();
    assert!(prompt.is_active());
  }

  #[test]
  fn test_handle_key_event() -> Result<()> {
    let (tx, _) = mpsc::unbounded_channel::<Message>();
    let mut prompt = EmailPrompt::new(tx);
    let event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());
    prompt.handle_key_event(event)?;
    Ok(())
  }

  #[test]
  fn test_validate() {
    let (tx, _) = mpsc::unbounded_channel::<Message>();
    let mut prompt = EmailPrompt::new(tx);
    prompt.validate();
  }
}
