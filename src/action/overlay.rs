use serde::{Deserialize, Serialize};

/// Represents different types of overlay components in the application.
///
/// This enum is used to manage the display and behavior of various overlay elements,
/// such as informational pop-ups, tooltips, or help screens. These overlays typically
/// appear above the main content and provide additional context or functionality.
///
/// Variants:
/// - `UsageInfo`: The default overlay type that provides usage information. This could
///   be used to display help text, user tips, or other relevant information that assists
///   users in navigating or understanding the application.
#[derive(
  Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum Overlay {
  #[default]
  UsageInfo,
}
