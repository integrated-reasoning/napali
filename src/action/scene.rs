use serde::{Deserialize, Serialize};

/// Represents distinct scenes or states within the application.
///
/// This enum is crucial for managing different contexts or pages the application
/// can display. Each scene represents a unique state or view within the application,
/// like different screens in a GUI or different states in a game or tool.
///
/// Variants:
/// - `Home`: The default scene, usually the starting point or main interface of the application.
///   It's typically where users land when they first launch the application.
/// - `Internals`: Represents a scene focused on displaying internal details or advanced settings
///   of the application. This might include configuration options, logs, or system statistics.
/// - `Session`: A scene that encapsulates an active user session, such as an ongoing task,
///   a workspace, or a user-specific interactive environment.
#[derive(
  Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum Scene {
  #[default]
  Home,
  Internals,
  Session,
}
