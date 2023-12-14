use crate::irx_client::api::ApiKey;
use color_eyre::eyre::Result;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};
use tokio::sync::mpsc;
use tracing::{self, instrument};

/// Represents the payload of a message in the application.
///
/// This enum encapsulates different types of data that can be sent as a message payload,
/// such as API keys, email addresses, or generic strings.
#[derive(
  Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum Payload {
  #[default]
  Empty,
  ApiKey(ApiKey),
  Email(EmailAddress),
  String(String),
}

/// Defines the possible addresses for message routing.
///
/// Addresses are used to identify different components or services in the application
/// that can send or receive messages.
#[derive(
  Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum Address {
  #[default]
  Drop,
  Router,
  IrxClient,
  Internals,
  StateDisplay,
  Session,
  Home,
  App,
}

/// Indicates whether a message is cacheable.
///
/// `Cacheable::Yes` suggests that the message can be cached for future use,
/// while `Cacheable::No` implies that the message should not be cached.
#[derive(
  Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum Cacheable {
  #[default]
  No,
  Yes,
}

/// Represents the type of a message.
///
/// `Kind::Tell` indicates a message that conveys information or a command,
/// while `Kind::Ask` represents a request for information.
#[derive(
  Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum Kind {
  #[default]
  Tell,
  Ask,
}

/// Struct for a message in the application's messaging system.
///
/// Contains details like the source and destination addresses, the payload,
/// optional tags, cacheability, and the kind of message.
#[derive(
  Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub struct Message {
  pub source: Address,
  pub destination: Address,
  pub payload: Payload,
  pub tag: Option<String>,
  pub cacheable: Cacheable,
  pub kind: Kind,
}

/// Type alias for a table mapping addresses to message senders.
type ChannelTable = HashMap<Address, mpsc::UnboundedSender<Message>>;

/// Represents a router in the messaging system.
///
/// The router is responsible for directing messages to the appropriate destination
/// based on the address.
#[derive(Debug)]
pub struct Router {
  channel_table: ChannelTable,
  message_rx_from_self: Option<mpsc::UnboundedReceiver<Message>>,
}

impl Router {
  /// Constructs a new `Router` with specified message channels.
  ///
  /// # Parameters
  ///
  /// * `tx`: The sender channel for the Router itself.
  ///
  /// # Returns
  ///
  /// `Result<(Self, mpsc::UnboundedSender<Message>)>` - A new Router instance and its sender.
  #[instrument]
  pub async fn new(
    tx: mpsc::UnboundedSender<Message>,
  ) -> Result<(Self, mpsc::UnboundedSender<Message>)> {
    let (message_tx_to_self, message_rx_from_self) =
      mpsc::unbounded_channel::<Message>();
    let mut channel_table = ChannelTable::new();
    channel_table.insert(Address::App, tx);
    channel_table.insert(Address::Router, message_tx_to_self.clone());
    Ok((
      Self {
        channel_table,
        message_rx_from_self: Some(message_rx_from_self),
      },
      message_tx_to_self,
    ))
  }

  /// Registers a sender for a specific address in the router's channel table.
  ///
  /// # Parameters
  ///
  /// * `addr`: The address to register.
  /// * `tx`: The sender channel associated with the address.
  pub fn register(
    &mut self,
    addr: Address,
    tx: mpsc::UnboundedSender<Message>,
  ) {
    self.channel_table.insert(addr, tx);
  }

  /// Starts the routing process for incoming messages.
  ///
  /// Listens for messages and routes them to the appropriate destination based on the address.
  pub fn run(&mut self) {
    let mut message_rx_from_self = self
      .message_rx_from_self
      .take()
      .expect("router has its own receiver"); // TODO replace all uses of expect()
    let channel_table = self.channel_table.clone();
    tokio::spawn(async move {
      loop {
        if let Some(message) = message_rx_from_self.recv().await {
          Self::route(message, &channel_table);
        }
      }
    });
  }

  /// Routes a message to the appropriate destination.
  ///
  /// # Parameters
  ///
  /// * `message`: The message to be routed.
  /// * `channel_table`: The table mapping addresses to message senders.
  fn route(message: Message, channel_table: &ChannelTable) {
    match channel_table.get(&message.destination) {
      Some(tx) => tx.send(message).expect("destination is reachable"),
      None => unreachable!(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use color_eyre::eyre::Result;

  #[tokio::test]
  async fn test_router_new() -> Result<()> {
    let (tx, _) = mpsc::unbounded_channel::<Message>();
    let _ = Router::new(tx).await?;
    Ok(())
  }

  #[tokio::test]
  async fn test_register() -> Result<()> {
    let (tx0, _) = mpsc::unbounded_channel::<Message>();
    let (tx1, _) = mpsc::unbounded_channel::<Message>();
    let (mut router, _) = Router::new(tx0).await?;
    router.register(Address::Drop, tx1);
    Ok(())
  }
}
