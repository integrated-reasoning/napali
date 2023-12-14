use crate::irx_client::api::ApiKey;
use crate::router::{Address, Cacheable, Kind, Message, Payload};
use color_eyre::eyre::{eyre, Result};
use email_address::EmailAddress;
use serde::Deserialize;
use std::{collections::HashMap, fs, str::FromStr};
use tokio::sync::mpsc;
use tracing::{self, instrument};
use url::Url;
pub mod api;

/// A client for interacting with the IRX API.
///
/// This structure manages the API key and handles message routing
/// between the client and the router.
#[derive(Debug)]
pub struct IrxClient {
  /// Optional API key for the client.
  pub api_key: Option<ApiKey>,
  /// Cloned sender for routing messages to the Router.
  message_tx_to_router: mpsc::UnboundedSender<Message>,
  /// Unique receiver for messages from the Router.
  message_rx_from_router: Option<mpsc::UnboundedReceiver<Message>>,
  /// Cloneable sender for sending messages to itself.
  pub message_tx_to_self: mpsc::UnboundedSender<Message>,
}

/// Response body structure received after registration.
#[derive(Deserialize, Debug)]
struct RegistrationResponseBody {
  #[serde(rename = "apiKeyId")]
  _api_key_id: String,
  #[serde(rename = "apiKeyValue")]
  api_key_value: ApiKey,
}

/// Structure representing the response from a registration request.
#[derive(Deserialize, Debug)]
struct RegistrationResponse {
  #[serde(rename = "statusCode")]
  _status_code: u32,
  body: String,
}

impl IrxClient {
  /// Base URL for the IRX API.
  const BASE: &str = "https://api.irx.sh/";

  /// Creates a new instance of `IrxClient`.
  ///
  /// This asynchronous function initializes the client, setting up the
  /// API key and message channels.
  ///
  /// # Arguments
  ///
  /// * `tx` - UnboundedSender for sending messages to the Router.
  ///
  /// # Returns
  ///
  /// A result containing the new `IrxClient` instance or an error.
  #[instrument]
  pub async fn new(tx: mpsc::UnboundedSender<Message>) -> Result<Self> {
    let api_key = match Self::read_api_key_from_config() {
      Some(key) => Some(key),
      None => match Self::request_new_api_key(None).await {
        Ok(key) => {
          Self::write_api_key_to_config(&key)?;
          Some(key)
        }
        Err(e) => {
          panic!("{e}");
        }
      },
    };
    let (message_tx_to_self, message_rx_from_router) =
      mpsc::unbounded_channel::<Message>();
    Ok(Self {
      api_key,
      message_tx_to_router: tx,
      message_rx_from_router: Some(message_rx_from_router),
      message_tx_to_self,
    })
  }

  /// Runs the message responder within the client.
  ///
  /// Listen for incoming messages from the router and
  /// handle them based on their kind.
  pub fn run_responder(&mut self) {
    let mut message_rx_from_router = self
      .message_rx_from_router
      .take()
      .expect("receiver is not None");
    let tx = self.message_tx_to_router.clone();
    let key = self.api_key.clone();

    tokio::spawn(async move {
      loop {
        if let Some(message) = message_rx_from_router.recv().await {
          match message.kind {
            // TODO HACK make this general
            Kind::Ask => {
              let response = Message {
                source: Address::IrxClient,
                destination: message.source,
                payload: (if let Some(k) = key.clone() {
                  Payload::ApiKey(k.clone())
                } else {
                  Payload::Empty
                }),
                tag: None,
                cacheable: Cacheable::No,
                kind: Kind::Tell,
              };
              tx.send(response).ok();
            }
            Kind::Tell => {
              if let Payload::Email(email) = message.payload {
                let _upgraded_key = // TODO use the new key
                  Self::request_new_api_key(Some(email)).await.unwrap(); // HACK don't unwrap
              };
            }
          }
        }
      }
    });
  }

  /// Reads the API key from the configuration file.
  ///
  /// This function attempts to retrieve the API key from the local
  /// configuration file, returning it if found.
  ///
  /// # Returns
  ///
  /// An option containing the `ApiKey` if found, or `None` otherwise.
  #[instrument]
  fn read_api_key_from_config() -> Option<ApiKey> {
    Self::get_config_path()
      .and_then(|path| fs::read_to_string(path.join("key.txt")).ok())
      .map(|s| s.replace('\n', ""))
      .and_then(|s| s.parse().ok())
      .or_else(|| {
        tracing::info!("api key not found on disk");
        None
      })
  }

  /// Retrieves the configuration path for the client.
  ///
  /// This function computes the path to the configuration directory
  /// where the API key is stored. It typically points to a `.config/irx`
  /// directory in the user's home folder.
  ///
  /// # Returns
  ///
  /// An option containing the `PathBuf` to the configuration directory
  /// if the home directory can be determined, or `None` otherwise.
  fn get_config_path() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|home| home.join(".config").join("irx"))
  }

  /// Writes the API key to the configuration file.
  ///
  /// This function saves the given API key to a file named `key.txt` within
  /// the configuration directory.
  ///
  /// # Arguments
  ///
  /// * `api_key` - A reference to the `ApiKey` to be saved.
  ///
  /// # Returns
  ///
  /// A result indicating the success or failure of the operation.
  #[instrument]
  fn write_api_key_to_config(api_key: &ApiKey) -> Result<()> {
    let config_path = Self::get_config_path()
      .ok_or_else(|| eyre!("failed to get config path"))?;
    let key_path = config_path.as_path().join(std::path::Path::new("key.txt"));
    fs::create_dir_all(config_path)?;
    fs::write(key_path, api_key.to_string())?;
    Ok(())
  }

  /// Requests a new API key from the IRX service.
  ///
  /// This asynchronous function makes a request to the IRX API to obtain
  /// a new API key. An optional email address can be provided to associate
  /// with the API key.
  ///
  /// # Arguments
  ///
  /// * `email` - An optional `EmailAddress` to be associated with the new API key.
  ///
  /// # Returns
  ///
  /// A result containing the new `ApiKey` or an error.
  #[instrument]
  async fn request_new_api_key(email: Option<EmailAddress>) -> Result<ApiKey> {
    let registration_key =
      ApiKey::from_str("ZtXHo0GHBX4PoDdHd2Gn27rsxGLoFVe086W7Zchk")
        .map_err(|e| eyre!(e))?;

    let client = reqwest::Client::new();
    let registration_url = Url::parse(Self::BASE)
      .expect("base url is valid")
      .join("register")
      .expect("registration url is valid");

    let request_base = client
      .post(registration_url)
      .header("x-api-key", registration_key.to_string());
    let request = match email {
      Some(address) => {
        let mut map = HashMap::new();
        map.insert("email", address.clone());
        request_base.json(&map)
      }
      None => request_base,
    };
    let registration_response: RegistrationResponse =
      request.send().await?.json::<RegistrationResponse>().await?;
    let body: RegistrationResponseBody =
      serde_json::from_str(&registration_response.body)?;
    Ok(body.api_key_value)
  }
}
