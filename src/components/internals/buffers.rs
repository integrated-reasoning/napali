use ringbuffer::ConstGenericRingBuffer;

/// Contains ring buffers for various types of application events and metrics.
///
/// This struct is used to store and manage historical data for different aspects of Napali,
/// such as tick counts, FPS metrics, and user actions. Each buffer is a fixed-size ring buffer.
/// The size of each buffer is chosen to be a power of 2 (e.g., 512), which is crucial for ensuring
/// efficient data management and constant-time access and insertion, optimizing performance.
#[derive(Default, Debug, Clone)]
pub struct Buffers {
  /// Buffer for storing tick counts. Size is a power of 2 for performance optimization.
  pub tick: ConstGenericRingBuffer<u32, 512>,
  /// Buffer for storing render tick counts.
  pub render_ticks: ConstGenericRingBuffer<u32, 512>,
  /// Buffer for storing render frames per second (FPS).
  pub render_fps: ConstGenericRingBuffer<f64, 512>,
  /// Buffer for storing application frames per second (FPS).
  pub app_fps: ConstGenericRingBuffer<f64, 512>,
  /// Buffer for storing window resize events.
  pub resize: ConstGenericRingBuffer<u32, 512>,
  /// Buffer for storing application suspend events.
  pub suspend: ConstGenericRingBuffer<u32, 512>,
  /// Buffer for storing application resume events.
  pub resume: ConstGenericRingBuffer<u32, 512>,
  /// Buffer for storing application quit events.
  pub quit: ConstGenericRingBuffer<u32, 512>,
  /// Buffer for storing screen refresh events.
  pub refresh: ConstGenericRingBuffer<u32, 512>,
  /// Buffer for storing error events.
  pub error: ConstGenericRingBuffer<u32, 512>,
  /// Buffer for storing scene change events.
  pub change_scene: ConstGenericRingBuffer<u32, 512>,
  /// Buffer for storing mode change events.
  pub change_mode: ConstGenericRingBuffer<u32, 512>,
  /// Buffer for storing view change events.
  pub change_view: ConstGenericRingBuffer<u32, 512>,
  /// Buffer for storing overlay toggle events.
  pub toggle_overlay: ConstGenericRingBuffer<u32, 512>,
  /// Buffer for storing help request events.
  pub help: ConstGenericRingBuffer<u32, 512>,
  /// Buffer for storing application trails as strings. Smaller size due to larger data per entry.
  pub trail: ConstGenericRingBuffer<String, 32>,
}
