use serde::{Deserialize, Serialize};

/// Represents the operational modes of the application.
///
/// This enum distinguishes between different interaction states the application can be in.
/// It's used to tailor the UI and behavior to the current context.
///
/// Variants:
/// - `Navigation`: The mode where the user navigates through the application.
///   In this mode, the focus is on moving around different UI elements or features.
/// - `TextInput`: A mode dedicated to text input. This is typically activated
///   when the user is expected to enter data, such as in a form or a text editor.
#[derive(
  Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum Mode {
  #[default]
  Navigation,
  TextInput,
}
