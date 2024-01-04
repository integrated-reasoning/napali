use clap::Parser;

/// Command-line interface (CLI) arguments for Napali.
///
/// This struct is defined using `clap`, a Rust crate for building CLIs with minimal boilerplate.
/// It parses command-line arguments and provides a user-friendly interface for configuring
/// Napali's settings.
#[derive(Parser, Debug)]
#[command(author, about)]
pub struct Cli {
  #[arg(
    short,
    long,
    value_name = "FILE",
    help = "The path to the MPS file to load"
  )]
  pub input_path: Option<String>,

  /// Tick rate configuration for Napali.
  ///
  /// Specifies the number of logic updates (ticks) per second.
  ///
  /// # Arguments
  ///
  /// * `-t`, `--tick_rate`: (Optional) The tick rate as a floating-point number.
  /// * `default_value_t = 1.0`: Default tick rate is set to 1.0 ticks per second.
  #[arg(
    short,
    long,
    value_name = "FLOAT",
    help = "Tick rate, i.e. number of ticks per second",
    default_value_t = 1.0
  )]
  pub tick_rate: f64,

  /// Frame rate configuration for Napali.
  ///
  /// Determines the number of frames rendered per second.
  ///
  /// # Arguments
  ///
  /// * `-f`, `--frame_rate`: (Optional) The frame rate as a floating-point number.
  /// * `default_value_t = 60.0`: Default frame rate is set to 60.0 frames per second.
  #[arg(
    short,
    long,
    value_name = "FLOAT",
    help = "Frame rate, i.e. number of frames per second",
    default_value_t = 60.0
  )]
  pub frame_rate: f64,

  /// Flag to enable or disable the Tokio console subscriber.
  ///
  /// When enabled, it activates the Tokio console subscriber for enhanced diagnostics
  /// and telemetry. Useful for debugging and performance monitoring in Tokio-based
  /// asynchronous applications.
  ///
  /// # Arguments
  ///
  /// * `-c`, `--console_subscriber`: (Optional) Boolean flag to enable the subscriber.
  /// * `default_value_t = false`: Disabled by default.
  #[arg(
    short,
    long,
    value_name = "BOOL",
    help = "Enable Tokio console subscriber",
    default_value_t = false
  )]
  pub console_subscriber: bool,
}
