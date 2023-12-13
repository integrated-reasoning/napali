use crate::router::{Address, Cacheable, Kind, Message, Payload};
use color_eyre::eyre::{eyre, Result};
use ratatui::{
  prelude::*,
  widgets::{Block, BorderType, Borders, Paragraph},
};
use std::collections::HashMap;
use tokio::sync::mpsc;

/// Manages the display of application state information in a TUI environment.
///
/// This struct is responsible for fetching and displaying various pieces of application state,
/// such as API keys and request counts. It uses asynchronous communication to retrieve data.
///
/// # Fields
/// - `message_tx_to_router`: Sender for passing messages to the router.
/// - `message_rx_from_router`: Receiver for messages from the router.
/// - `message_tx_to_self`: Sender for passing messages to the `StateDisplay` component itself.
/// - `cache`: A cache for storing fetched state data.
#[derive(Debug)]
pub struct StateDisplay {
  message_tx_to_router: mpsc::UnboundedSender<Message>,
  message_rx_from_router: Option<mpsc::UnboundedReceiver<Message>>,
  pub message_tx_to_self: mpsc::UnboundedSender<Message>,
  cache: HashMap<String, String>,
}

impl StateDisplay {
  /// Constructs a new instance of `StateDisplay`.
  ///
  /// Initializes the state display with message channels for communication and an empty cache.
  ///
  /// # Arguments
  /// - `tx`: Sender for passing messages to the router.
  ///
  /// # Returns
  /// A new instance of `StateDisplay`.
  pub fn new(tx: mpsc::UnboundedSender<Message>) -> StateDisplay {
    let (message_tx_to_self, message_rx_from_router) =
      mpsc::unbounded_channel::<Message>();
    StateDisplay {
      message_tx_to_router: tx,
      message_rx_from_router: Some(message_rx_from_router),
      message_tx_to_self,
      cache: HashMap::new(),
    }
  }

  /// Synchronously requests and retrieves the API key using the provided runtime handle.
  ///
  /// # Arguments
  /// - `handle`: The Tokio runtime handle used for asynchronous operations.
  ///
  /// # Returns
  /// The retrieved API key as a `String`, or an error if the operation fails.
  pub fn ask_for_key_sync(
    &mut self,
    handle: tokio::runtime::Handle,
  ) -> Result<String> {
    let request = Message {
      source: Address::StateDisplay,
      destination: Address::IrxClient,
      payload: Payload::Empty,
      tag: None,
      cacheable: Cacheable::No,
      kind: Kind::Ask,
    };

    self.message_tx_to_router.send(request)?;

    let mut response_receiver = self
      .message_rx_from_router
      .take()
      .ok_or_else(|| eyre!("failed to take ownership of receiver"))?;

    let (api_key, receiver) = futures::executor::block_on(async {
      handle
        .spawn(async {
          match response_receiver.recv().await {
            Some(message) => match message.payload {
              Payload::ApiKey(key) => Ok((key.to_string(), response_receiver)),
              _ => Err(eyre!("invalid payload")),
            },
            None => Err(eyre!("no message received")),
          }
        })
        .await?
    })?;
    self.message_rx_from_router = Some(receiver);
    Ok(api_key)
  }

  /// Retrieves the unbounded sender handle for the state display.
  ///
  /// # Returns
  /// An `UnboundedSender<Message>` for sending messages.
  pub fn get_tx_handle(&self) -> mpsc::UnboundedSender<Message> {
    self.message_tx_to_self.clone()
  }

  /// Creates a paragraph widget for displaying the application's state.
  ///
  /// Fetches and displays various pieces of state information, including the API key.
  ///
  /// # Returns
  /// A `Paragraph` widget configured to display the state information.
  fn state_display(&mut self) -> Result<Paragraph<'_>> {
    let api_key = if let Some(key) = self.cache.get("api_key") {
      key.clone()
    } else {
      let key = self.ask_for_key_sync(tokio::runtime::Handle::current())?;
      self.cache.insert(String::from("api_key"), key.clone());
      key
    };
    // TODO: Don't render the key in plain text by default
    let time = chrono::Utc::now();
    let text = vec![
      Line::from(format!("{time}")),
      Line::from(""),
      Line::from("IRX Client:"),
      Line::from(format!("  API key: {api_key}")),
      Line::from("    Found: ?".to_string()),
      Line::from("    Path: ?".to_string()),
      Line::from("    Value: ?".to_string()),
      Line::from("    Tier: ?".to_string()),
      Line::from("  Requests:"),
      Line::from("    This session: ?".to_string()),
      Line::from("    Lifetime: ?".to_string()),
    ];
    Ok(
      Paragraph::new(text)
        .block(
          Block::default()
            .title("State")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default())
            .border_type(BorderType::Rounded),
        )
        //.style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Left),
    )
  }

  /// Determines the layout area for rendering the state display.
  ///
  /// # Arguments
  /// - `area`: The `Rect` representing the entire renderable area.
  ///
  /// # Returns
  /// A `Rect` defining the area for the state display.
  fn layer(area: Rect) -> Rect {
    area
  }

  /// Renders the state display onto the specified area of the frame.
  ///
  /// # Arguments
  /// - `area`: The area where the state display should be rendered.
  /// - `f`: Mutable reference to the frame for rendering.
  ///
  /// # Returns
  /// `Ok(())` on successful rendering, or an error in case of failure.
  pub fn render(&mut self, area: Rect, f: &mut Frame<'_>) -> Result<()> {
    let layer = Self::layer(area);
    let text = self.state_display()?;
    f.render_widget(text, layer);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_state_display_new() {
    let (tx, _) = mpsc::unbounded_channel::<Message>();
    StateDisplay::new(tx);
  }
}
