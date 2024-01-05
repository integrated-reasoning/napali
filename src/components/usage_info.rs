use super::{Component, Frame, State};
use crate::action::overlay::Overlay;
use crate::{action::Action, config::Config};
use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{
  prelude::*,
  widgets::{block::Block, BorderType, Borders, Clear, Row, Table},
};
use tokio::sync::mpsc::UnboundedSender;

/// Manages the display of usage information and keybindings in Napali.
///
/// This struct handles interactions and state for displaying a helpful guide of keybindings
/// and their respective actions within the application.
#[derive(Default, Debug, Clone)]
pub struct UsageInfo {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
  state: State,
}

impl Component for UsageInfo {
  /// Registers an action handler for sending actions.
  ///
  /// # Arguments
  /// - `tx`: The sender for dispatching actions.
  fn register_action_handler(
    &mut self,
    tx: UnboundedSender<Action>,
  ) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  /// Registers a configuration handler.
  ///
  /// # Arguments
  /// - `config`: The application configuration.
  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    self.config = config;
    Ok(())
  }

  /// Handles key events.
  ///
  /// # Arguments
  /// - `_key`: The key event to handle.
  fn handle_key_events(&mut self, _key: KeyEvent) -> Result<Option<Action>> {
    self.state = State::Hidden;
    Ok(None)
  }

  /// Updates the state based on the received action.
  ///
  /// # Arguments
  /// - `action`: The action received by the component.
  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    if let Action::ToggleOverlay(overlay) = action {
      match overlay {
        Overlay::UsageInfo => {
          self.state = match self.state {
            State::Visible => State::Hidden,
            State::Hidden => State::Visible,
          };
        }
      }
    }
    Ok(None)
  }

  /// Draws the usage information on the terminal frame.
  ///
  /// # Arguments
  /// - `f`: The frame to draw onto.
  /// - `area`: The area where the component should be rendered.
  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    if self.state != State::Hidden {
      let rect = area.inner(&Margin {
        horizontal: 4,
        vertical: 4,
      });
      let vertical_rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
          Constraint::Ratio(2, 7),
          Constraint::Ratio(3, 7),
          Constraint::Ratio(2, 7),
        ])
        .split(rect);
      let horizontal_rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
          Constraint::Ratio(2, 7),
          Constraint::Ratio(3, 7),
          Constraint::Ratio(2, 7),
        ])
        .split(vertical_rects[1]);
      let rows = vec![
        // TODO: Make this component-specific
        Row::new(vec!["d", "Data"]),
        Row::new(vec!["s", "Session"]),
        Row::new(vec!["i", "Internals"]),
        Row::new(vec!["e", "Email prompt"]),
        Row::new(vec!["a", "About"]),
        Row::new(vec!["q", "Quit"]),
        Row::new(vec!["?", "Show usage help"]),
      ];
      let table = Table::new(
        rows,
        [Constraint::Percentage(10), Constraint::Percentage(90)],
      )
      .header(
        Row::new(vec!["Key", "Action"])
          .bottom_margin(1)
          .style(Style::default().add_modifier(Modifier::BOLD)),
      )
      .column_spacing(1)
      .block(
        Block::default()
          .title("Usage")
          .title_alignment(Alignment::Left)
          .borders(Borders::ALL)
          .border_style(Style::default())
          .border_type(BorderType::Rounded)
          .style(Style::default()),
      )
      .style(Style::default());
      f.render_widget(Clear, horizontal_rects[1]);
      f.render_widget(
        table,
        horizontal_rects[1].inner(&Margin {
          vertical: 1,
          horizontal: 1,
        }),
      );
    }
    Ok(())
  }
}
