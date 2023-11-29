use ratatui::{
  prelude::*,
  widgets::{Block, BorderType, Borders},
};

/// Represents the Workspaces section in a TUI application.
///
/// This struct manages the display of workspace-related information, encapsulating a `Block` widget.
#[derive(Debug)]
pub struct Workspaces<'a> {
  pub block: Block<'a>,
}

impl<'a> Workspaces<'a> {
  /// Constructs a new `Workspaces` instance with default settings.
  pub fn new() -> Workspaces<'a> {
    Workspaces {
      block: Block::default()
        .title("Workspaces")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_style(Style::default())
        .border_type(BorderType::Rounded),
    }
  }
}

/// Represents the Status section in a TUI application.
///
/// This struct is responsible for displaying the current status, using a `Block` widget for visualization.
#[derive(Debug)]
pub struct Status<'a> {
  pub block: Block<'a>,
}

impl<'a> Status<'a> {
  /// Constructs a new `Status` instance with default settings.
  pub fn new() -> Status<'a> {
    Status {
      block: Block::default()
        .title("Status")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_style(Style::default())
        .border_type(BorderType::Rounded),
    }
  }
}

/// Represents the Plots section in a TUI application.
///
/// This struct manages the display of plot-related data, encapsulating a `Block` widget.
#[derive(Debug)]
pub struct Plots<'a> {
  pub block: Block<'a>,
}

impl<'a> Plots<'a> {
  /// Constructs a new `Plots` instance with default settings.
  pub fn new() -> Plots<'a> {
    Plots {
      block: Block::default()
        .title("Plot A")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_style(Style::default())
        .border_type(BorderType::Rounded),
    }
  }
}

/// Represents the Logs section in a TUI application.
///
/// This struct is used for displaying logs, using a `Block` widget for the UI.
#[derive(Debug)]
pub struct Logs<'a> {
  pub block: Block<'a>,
}

impl<'a> Logs<'a> {
  /// Constructs a new `Logs` instance with default settings.
  pub fn new() -> Logs<'a> {
    Logs {
      block: Block::default()
        .title("Logs")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_style(Style::default())
        .border_type(BorderType::Rounded),
    }
  }
}
