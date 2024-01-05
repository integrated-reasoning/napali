pub mod mode;
pub mod overlay;
pub mod scene;
pub mod view;
use mode::Mode;
use overlay::Overlay;
use scene::Scene;
use serde::{
  de::{self, Deserializer, Visitor},
  Deserialize, Serialize,
};
use std::fmt;
use strum::EnumIter;
use view::View;

/// Represents possible actions within the application.
///
/// This enum defines various actions that can be triggered by the user or the system,
/// such as rendering, resizing, or changing the current scene.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumIter)]
pub enum Action {
  #[default]
  Tick,
  Render,
  Resize(u16, u16),
  Suspend,
  Resume,
  Quit,
  Refresh,
  Error(String),
  ChangeScene(Scene),
  ChangeView(View),
  ToggleOverlay(Overlay),
  ChangeMode(Mode),
  Help,
}

impl<'de> Deserialize<'de> for Action {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    /// A visitor to assist in deserializing `Action`.
    struct ActionVisitor;

    impl<'de> Visitor<'de> for ActionVisitor {
      type Value = Action;

      /// Specifies the expected format for the `Action` enum during deserialization.
      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid string representation of Action")
      }

      /// Visits a string to deserialize it into an `Action`.
      ///
      /// This method matches the provided string with known `Action` variants.
      /// It handles custom parsing for variants like `Resize` and `Error`.
      fn visit_str<E>(self, value: &str) -> Result<Action, E>
      where
        E: de::Error,
      {
        match value {
          // Match each action variant with its string representation.
          "Tick" => Ok(Action::Tick),
          "Render" => Ok(Action::Render),
          "Suspend" => Ok(Action::Suspend),
          "Resume" => Ok(Action::Resume),
          "Quit" => Ok(Action::Quit),
          "Refresh" => Ok(Action::Refresh),
          "ChangeScene(About)" => Ok(Action::ChangeScene(Scene::About)),
          "ChangeScene(Internals)" => Ok(Action::ChangeScene(Scene::Internals)),
          "ChangeScene(Session)" => Ok(Action::ChangeScene(Scene::Session)),
          "ChangeView(A)" => Ok(Action::ChangeView(View::A)),
          "ChangeView(R)" => Ok(Action::ChangeView(View::R)),
          "ChangeView(L)" => Ok(Action::ChangeView(View::L)),
          "ChangeView(Prompt)" => Ok(Action::ChangeView(View::Prompt)),
          "ToggleOverlay(UsageInfo)" => {
            Ok(Action::ToggleOverlay(Overlay::UsageInfo))
          }
          "Help" => Ok(Action::Help),
          data if data.starts_with("Error(") => {
            let error_msg =
              data.trim_start_matches("Error(").trim_end_matches(')');
            Ok(Action::Error(error_msg.to_string()))
          }
          data if data.starts_with("Resize(") => {
            let parts: Vec<&str> = data
              .trim_start_matches("Resize(")
              .trim_end_matches(')')
              .split(',')
              .collect();
            if parts.len() == 2 {
              let width: u16 = parts[0].trim().parse().map_err(E::custom)?;
              let height: u16 = parts[1].trim().parse().map_err(E::custom)?;
              Ok(Action::Resize(width, height))
            } else {
              Err(E::custom(format!("Invalid Resize format: {value}")))
            }
          }
          _ => Err(E::custom(format!("Unknown Action variant: {value}"))),
        }
      }
    }

    deserializer.deserialize_str(ActionVisitor)
  }
}
