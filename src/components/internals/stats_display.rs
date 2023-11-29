use super::buffers::Buffers;
use super::counters::Counters;
use crate::router::Message;
use crate::{action::Action, tui::Frame};
use color_eyre::eyre::Result;
use ratatui::{
  prelude::*,
  widgets::{Block, Borders},
};
use ringbuffer::RingBuffer;
use std::time::Instant;
use tokio::sync::mpsc;
use tui_tree_widget::{Tree, TreeItem, TreeState};

// TODO: Detect when Render FPS < target FPS and adjust target accordingly
// To test, start the app on a lower-res monitor and move it to a high res one, then move it back
// Building in release mode apperas to improve FPS significantly
// TODO: Remove all uses of expect, which panics
// (also ensure unwrap is not used in the release)

/// Manages and displays statistics of application events in a TUI environment.
///
/// Tracks various events and metrics, updating and visualizing them using tree widgets.
#[derive(Debug)]
pub struct StatsDisplay {
  app_start_time: Instant,
  render_start_time: Instant,
  pub message_tx_to_self: mpsc::UnboundedSender<Message>,
}

impl StatsDisplay {
  /// Constructs a new `StatsDisplay`.
  ///
  /// Initializes the starting times for application and rendering FPS calculations.
  pub fn new() -> StatsDisplay {
    let (message_tx_to_self, _) = mpsc::unbounded_channel::<Message>();
    StatsDisplay {
      app_start_time: Instant::now(),
      render_start_time: Instant::now(),
      message_tx_to_self,
    }
  }

  /// Updates various counters based on the received action.
  ///
  /// # Arguments
  /// - `actions`: The `Buffers` containing the event data.
  /// - `counters`: The `Counters` tracking the number of events.
  /// - `action`: The action received by the application.
  pub fn update(
    actions: &mut Buffers,
    counters: &mut Counters,
    action: Action,
  ) {
    match action {
      Action::Tick => {
        counters.tick = counters.tick.saturating_add(1);
      }
      Action::Render => {
        counters.render = counters.render.saturating_add(1);

        if counters.tick > 1 {
          actions.tick.push(counters.tick);
          actions.render_ticks.push(counters.render);
          actions.app_fps.push(counters.app_fps);
          actions.render_fps.push(counters.render_fps);
          actions.resize.push(counters.resize);
          actions.suspend.push(counters.suspend);
          actions.resume.push(counters.resume);
          actions.quit.push(counters.quit);
          actions.refresh.push(counters.refresh);
          actions.error.push(counters.error);
          actions.change_scene.push(counters.change_scene);
          actions.change_mode.push(counters.change_mode);
          actions.change_view.push(counters.change_view);
          actions.toggle_overlay.push(counters.toggle_overlay);
          actions.help.push(counters.help);
        }
      }
      Action::Resize(_, _) => {
        counters.resize = counters.resize.saturating_add(1);
      }
      Action::Suspend => {
        counters.suspend = counters.suspend.saturating_add(1);
      }
      Action::Resume => {
        counters.resume = counters.resume.saturating_add(1);
      }
      Action::Quit => {
        counters.quit = counters.quit.saturating_add(1);
      }
      Action::Refresh => {
        counters.refresh = counters.refresh.saturating_add(1);
      }
      Action::Error(_) => {
        counters.error = counters.error.saturating_add(1);
      }
      Action::ChangeScene(_) => {
        counters.change_scene = counters.change_scene.saturating_add(1);
      }
      Action::ChangeMode(_) => {
        counters.change_mode = counters.change_mode.saturating_add(1);
      }
      Action::ChangeView(_) => {
        counters.change_view = counters.change_view.saturating_add(1);
      }
      Action::ToggleOverlay(_) => {
        counters.toggle_overlay = counters.toggle_overlay.saturating_add(1);
      }
      Action::Help => {
        counters.help = counters.help.saturating_add(1);
      }
    }
    actions.trail.push(format!(
      "{:?} {:?}",
      chrono::Utc::now(),
      action.clone()
    ));
  }

  /// Calculates and updates the application FPS counter.
  ///
  /// # Arguments
  /// - `counters`: The `Counters` tracking the number of events.
  pub fn app_tick(&mut self, counters: &mut Counters) {
    counters.app_frames += 1;
    let now = Instant::now();
    let elapsed = (now - self.app_start_time).as_secs_f64();
    if elapsed >= 1.0 {
      counters.app_fps = f64::from(counters.app_frames) / elapsed;
      self.app_start_time = now;
      counters.app_frames = 0;
    }
  }

  /// Calculates and updates the rendering FPS counter.
  ///
  /// # Arguments
  /// - `counters`: The `Counters` tracking the number of events.
  pub fn render_tick(&mut self, counters: &mut Counters) {
    counters.render_frames += 1;
    let now = Instant::now();
    let elapsed = (now - self.render_start_time).as_secs_f64();
    if elapsed >= 1.0 {
      counters.render_fps = f64::from(counters.render_frames) / elapsed;
      self.render_start_time = now;
      counters.render_frames = 0;
    }
  }

  /// Creates a tree widget for displaying statistics.
  ///
  /// # Arguments
  /// - `counters`: The `Counters` containing statistical data.
  ///
  /// # Returns
  /// A `Result` containing the tree widget and its state or an error.
  fn tree(counters: &Counters) -> Result<(Tree<usize>, TreeState<usize>)> {
    let mut state = TreeState::default();
    let nodes = TreeItem::new(
      1,
      "Actions",
      vec![
        TreeItem::new_leaf(2, format!("Tick: {}", counters.tick)),
        TreeItem::new_leaf(3, format!("Render: {}", counters.render)),
        TreeItem::new_leaf(4, format!("Resize: {}", counters.resize)),
        TreeItem::new_leaf(5, format!("Suspend: {}", counters.suspend)),
        TreeItem::new_leaf(6, format!("Resume: {}", counters.resume)),
        TreeItem::new_leaf(7, format!("Quit: {}", counters.quit)),
        TreeItem::new_leaf(8, format!("Refresh: {}", counters.refresh)),
        TreeItem::new_leaf(9, format!("Error: {}", counters.error)),
        TreeItem::new_leaf(
          10,
          format!("ChangeScene: {}", counters.change_scene),
        ),
        TreeItem::new_leaf(11, format!("ChangeMode: {}", counters.change_mode)),
        TreeItem::new_leaf(12, format!("ChangeView: {}", counters.change_view)),
        TreeItem::new_leaf(
          13,
          format!("ToggleOverlay: {}", counters.toggle_overlay),
        ),
        TreeItem::new_leaf(14, format!("Help: {}", counters.help)),
      ],
    )?;
    //let root = TreeItem::new(0, "TUI", vec![actions])?;
    let items = vec![nodes];
    state.open(vec![1]);
    Ok((
      Tree::new(items)
        .expect("all item identifiers are unique")
        .block(Block::new().borders(Borders::ALL).title("Stats")),
      state,
    ))
  }

  /// Determines the layout area for the statistics display.
  ///
  /// # Arguments
  /// - `area`: The `Rect` representing the renderable area.
  ///
  /// # Returns
  /// A `Rect` defining the area for the stats display.
  fn layer(area: Rect) -> Rect {
    area
  }

  /// Renders the statistics display in the specified area of the frame.
  ///
  /// # Arguments
  /// - `counters`: The `Counters` containing statistical data.
  /// - `area`: The area where the stats should be rendered.
  /// - `f`: The frame for rendering.
  ///
  /// # Returns
  /// `Ok(())` on successful rendering, or an error in case of failure.
  pub fn render(
    counters: &Counters,
    area: Rect,
    f: &mut Frame<'_>,
  ) -> Result<()> {
    let layer = Self::layer(area);
    let (tree_widget, mut state) = Self::tree(counters)?;
    f.render_stateful_widget(tree_widget, layer, &mut state);
    Ok(())
  }
}
