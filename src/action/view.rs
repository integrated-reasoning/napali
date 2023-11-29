use serde::{Deserialize, Serialize};

/// Represents the various views or visual states within a particular scene of the application.
///
/// This enum is used to switch between different layouts or perspectives within a given scene,
/// allowing the application to present information in various formats or contexts.
/// Each variant represents a unique view, possibly with its own UI elements and interaction modes.
#[derive(
  Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum View {
  #[default]
  A,
  L,
  R,
  Prompt,
}
