use serde::{Deserialize, Serialize};

/// Represents a collection of counters for tracking different events and metrics in Napali.
///
/// This struct is used to record various occurrences and performance metrics,
/// such as the number of ticks, renderings, and user interactions.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Counters {
  /// Counter for application ticks.
  pub tick: u32,
  /// Counter for render events.
  pub render: u32,
  /// Current render frames per second (FPS).
  pub render_fps: f64,
  /// Total number of frames rendered.
  pub render_frames: u32,
  /// Current application frames per second (FPS).
  pub app_fps: f64,
  /// Total number of frames processed by Napali.
  pub app_frames: u32,
  /// Counter for window resize events.
  pub resize: u32,
  /// Counter for application suspend events.
  pub suspend: u32,
  /// Counter for application resume events.
  pub resume: u32,
  /// Counter for application quit events.
  pub quit: u32,
  /// Counter for screen refresh events.
  pub refresh: u32,
  /// Counter for error events.
  pub error: u32,
  /// Counter for scene change events.
  pub change_scene: u32,
  /// Counter for mode change events.
  pub change_mode: u32,
  /// Counter for view change events.
  pub change_view: u32,
  /// Counter for overlay toggle events.
  pub toggle_overlay: u32,
  /// Counter for help request events.
  pub help: u32,
}
