use color_eyre::eyre::Result;
use crossterm::{
  cursor,
  event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste,
    EnableMouseCapture, Event as CrosstermEvent, KeyEvent, KeyEventKind,
    MouseEvent,
  },
  terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::backend::CrosstermBackend as Backend;
use serde::{Deserialize, Serialize};
use std::{
  ops::{Deref, DerefMut},
  time::Duration,
};
use tokio::{
  sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
  task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

/// Type alias for standard error output.
pub type IO = std::io::Stderr;

/// Returns the standard error output stream.
pub fn io() -> IO {
  std::io::stderr()
}

/// Type alias for a frame in the TUI.
pub type Frame<'a> = ratatui::Frame<'a>;

/// Enum representing various types of events in the TUI.
///
/// Events include lifecycle events like initialization and quitting, UI updates,
/// input events, and error handling.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
  Init,
  Quit,
  Error,
  Closed,
  Tick,
  Render,
  FocusGained,
  FocusLost,
  Paste(String),
  Key(KeyEvent),
  Mouse(MouseEvent),
  Resize(u16, u16),
}

/// The main structure for the Terminal User Interface (TUI).
///
/// Manages the terminal, event handling, and rendering for the TUI application.
pub struct Tui {
  pub terminal: ratatui::Terminal<Backend<IO>>,
  pub task: JoinHandle<()>,
  pub cancellation_token: CancellationToken,
  pub event_rx: UnboundedReceiver<Event>,
  pub event_tx: UnboundedSender<Event>,
  pub frame_rate: f64,
  pub tick_rate: f64,
  pub mouse: bool,
  pub paste: bool,
}

impl Tui {
  /// Creates a new instance of the TUI.
  ///
  /// Initializes the terminal and event handling components.
  ///
  /// # Returns
  ///
  /// `Result<Self>` - The new TUI instance or an error.
  pub fn new() -> Result<Self> {
    let tick_rate = 4.0;
    let frame_rate = 60.0;
    let terminal = ratatui::Terminal::new(Backend::new(io()))?;
    let (event_tx, event_rx) = mpsc::unbounded_channel();
    let cancellation_token = CancellationToken::new();
    let task = tokio::spawn(async {});
    let mouse = false;
    let paste = false;
    Ok(Self {
      terminal,
      task,
      cancellation_token,
      event_rx,
      event_tx,
      frame_rate,
      tick_rate,
      mouse,
      paste,
    })
  }

  /// Sets the tick rate for the TUI.
  ///
  /// # Parameters
  ///
  /// * `tick_rate`: The desired number of logic updates per second.
  ///
  /// # Returns
  ///
  /// Self - The modified Tui instance.
  pub fn tick_rate(mut self, tick_rate: f64) -> Self {
    self.tick_rate = tick_rate;
    self
  }

  /// Sets the frame rate for the TUI.
  ///
  /// # Parameters
  ///
  /// * `frame_rate`: The desired number of frames rendered per second.
  ///
  /// # Returns
  ///
  /// Self - The modified Tui instance.
  pub fn frame_rate(mut self, frame_rate: f64) -> Self {
    self.frame_rate = frame_rate;
    self
  }

  /// Enables or disables mouse event capture.
  ///
  /// # Parameters
  ///
  /// * `mouse`: Boolean flag to enable (`true`) or disable (`false`) mouse events.
  ///
  /// # Returns
  ///
  /// Self - The modified Tui instance.
  pub fn _mouse(mut self, mouse: bool) -> Self {
    self.mouse = mouse;
    self
  }

  pub fn _paste(mut self, paste: bool) -> Self {
    self.paste = paste;
    self
  }

  /// Starts the event loop for the TUI.
  ///
  /// Initiates handling of UI events and manages tick and render intervals.
  pub fn start(&mut self) {
    let tick_delay = std::time::Duration::from_secs_f64(1.0 / self.tick_rate);
    let render_delay =
      std::time::Duration::from_secs_f64(1.0 / self.frame_rate);
    self.cancel();
    self.cancellation_token = CancellationToken::new();
    let cancellation_token = self.cancellation_token.clone();
    let event_tx = self.event_tx.clone();
    self.task = tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      let mut tick_interval = tokio::time::interval(tick_delay);
      let mut render_interval = tokio::time::interval(render_delay);
      event_tx.send(Event::Init).unwrap();
      loop {
        let tick_delay = tick_interval.tick();
        let render_delay = render_interval.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
          () = cancellation_token.cancelled() => {
            break;
          }
          maybe_event = crossterm_event => {
            match maybe_event {
              Some(Ok(evt)) => {
                match evt {
                  CrosstermEvent::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                      event_tx.send(Event::Key(key)).unwrap();
                    }
                  },
                  CrosstermEvent::Mouse(mouse) => {
                    event_tx.send(Event::Mouse(mouse)).unwrap();
                  },
                  CrosstermEvent::Resize(x, y) => {
                    event_tx.send(Event::Resize(x, y)).unwrap();
                  },
                  CrosstermEvent::FocusLost => {
                    event_tx.send(Event::FocusLost).unwrap();
                  },
                  CrosstermEvent::FocusGained => {
                    event_tx.send(Event::FocusGained).unwrap();
                  },
                  CrosstermEvent::Paste(s) => {
                    event_tx.send(Event::Paste(s)).unwrap();
                  },
                }
              }
              Some(Err(_)) => {
                event_tx.send(Event::Error).unwrap();
              }
              None => {},
            }
          },
          _ = tick_delay => {
              event_tx.send(Event::Tick).unwrap();
          },
          _ = render_delay => {
              event_tx.send(Event::Render).unwrap();
          },
        }
      }
    });
  }

  /// Stops the event loop and any ongoing tasks.
  ///
  /// Ensures a clean shutdown of the TUI's event loop and associated tasks.
  pub fn stop(&self) {
    self.cancel();
    let mut counter = 0;
    while !self.task.is_finished() {
      std::thread::sleep(Duration::from_millis(1));
      counter += 1;
      if counter > 50 {
        self.task.abort();
      }
      if counter > 100 {
        log::error!(
          "Failed to abort task in 100 milliseconds for unknown reason"
        );
        break;
      }
    }
  }

  /// Enters the TUI mode, setting up the terminal and enabling raw mode.
  ///
  /// # Returns
  ///
  /// `Result<()>` - Ok if the terminal is successfully set up, or an error.
  pub fn enter(&mut self) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(io(), EnterAlternateScreen, cursor::Hide)?;
    if self.mouse {
      crossterm::execute!(io(), EnableMouseCapture)?;
    }
    if self.paste {
      crossterm::execute!(io(), EnableBracketedPaste)?;
    }
    self.start();
    Ok(())
  }

  /// Exits the TUI mode, restoring the terminal to its original state.
  ///
  /// # Returns
  ///
  /// `Result<()>` - Ok if the terminal is successfully restored, or an error.
  pub fn exit(&mut self) -> Result<()> {
    self.stop();
    if crossterm::terminal::is_raw_mode_enabled()? {
      self.flush()?;
      if self.paste {
        crossterm::execute!(io(), DisableBracketedPaste)?;
      }
      if self.mouse {
        crossterm::execute!(io(), DisableMouseCapture)?;
      }
      crossterm::execute!(io(), LeaveAlternateScreen, cursor::Show)?;
      crossterm::terminal::disable_raw_mode()?;
    }
    Ok(())
  }

  /// Cancels any ongoing tasks or operations in the TUI.
  pub fn cancel(&self) {
    self.cancellation_token.cancel();
  }

  /// Suspends the TUI, typically in response to a system signal.
  ///
  /// # Returns
  ///
  /// `Result<()>` - Ok if the TUI is successfully suspended, or an error.
  pub fn suspend(&mut self) -> Result<()> {
    self.exit()?;
    #[cfg(not(windows))]
    signal_hook::low_level::raise(signal_hook::consts::signal::SIGTSTP)?;
    Ok(())
  }

  /// Resumes the TUI after being suspended.
  ///
  /// # Returns
  ///
  /// `Result<()>` - Ok if the TUI is successfully resumed, or an error.
  pub fn _resume(&mut self) -> Result<()> {
    self.enter()?;
    Ok(())
  }

  /// Fetches the next event from the TUI event stream.
  ///
  /// # Returns
  ///
  /// `Option<Event>` - The next event if available, or `None`.
  pub async fn next(&mut self) -> Option<Event> {
    self.event_rx.recv().await
  }
}

impl Deref for Tui {
  type Target = ratatui::Terminal<Backend<IO>>;

  fn deref(&self) -> &Self::Target {
    &self.terminal
  }
}

impl DerefMut for Tui {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.terminal
  }
}

impl Drop for Tui {
  /// Ensures a clean exit when the Tui instance is dropped.
  ///
  /// Automatically exits raw mode and cleans up the terminal state.
  fn drop(&mut self) {
    self.exit().unwrap();
  }
}
