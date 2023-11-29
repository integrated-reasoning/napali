use crate::action::view;
use ratatui::{
  prelude::*,
  widgets::{Block, BorderType, Borders, Tabs},
};

/// Represents different views that can be displayed in the Jobs section.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum View {
  #[default]
  All,
  Remote,
  Local,
  Prompt,
}

impl From<View> for usize {
  fn from(v: View) -> usize {
    match v {
      View::All => 0,
      View::Remote => 1,
      View::Local => 2,
      View::Prompt => 0, // HACK: Temporary solution
    }
  }
}

impl From<view::View> for View {
  fn from(k: view::View) -> View {
    match k {
      view::View::A => View::All,
      view::View::R => View::Remote,
      view::View::L => View::Local,
      view::View::Prompt => View::Prompt, // HACK: Temporary solution
    }
  }
}

/// Manages and displays a tab bar for different job views in a TUI application.
#[derive(Debug)]
pub struct Jobs<'a> {
  block: Block<'a>,
  view: View,
}

impl<'a> Jobs<'a> {
  /// Constructs a new `Jobs` instance with default settings.
  pub fn new() -> Jobs<'a> {
    Jobs {
      block: Block::default()
        .title("Jobs")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_style(Style::default())
        .border_type(BorderType::Rounded),
      view: View::default(),
    }
  }

  /// Creates a tab bar widget based on the current view.
  fn tab_bar_widget(&self) -> Tabs<'a> {
    let job_tab_titles = match self.view {
      View::All => vec!["All", "Remote", "Local"],
      View::Remote => vec!["Remote", "Local", "All"],
      View::Local => vec!["Local", "All", "Remote"],
      View::Prompt => unreachable!(), // HACK
    };
    Tabs::new(
      job_tab_titles
        .iter()
        .map(|t| {
          let (first, rest) = t.split_at(1);
          Line::from(vec![first.white(), rest.gray()])
        })
        .collect(),
    )
    .block(Block::default().borders(Borders::NONE))
    .select(0)
    .style(Style::default())
    .highlight_style(Style::default().bold())
  }

  /// Calculates layout areas for different parts of the Jobs display.
  fn layers(area: Rect) -> (Rect, Rect) {
    let jobs_bar = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![Constraint::Max(3), Constraint::Min(1)])
      .split(area.inner(&Margin {
        horizontal: 1,
        vertical: 1,
      }));
    (area, jobs_bar[0])
  }

  /// Sets the current view for the Jobs display.
  pub fn set_view(&mut self, k: view::View) {
    self.view = View::from(k);
  }

  /// Renders the Jobs display in the specified area of the frame.
  pub fn render(&mut self, area: Rect, f: &mut Frame<'_>) {
    let (main_area, tab_bar_area) = Self::layers(area);
    let tab_bar = self.tab_bar_widget();

    // Render the main block and the tab bar in their respective areas
    f.render_widget(self.block.clone(), main_area);
    f.render_widget(tab_bar, tab_bar_area);
  }
}
