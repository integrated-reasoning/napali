use color_eyre::{eyre::eyre, Report, Result};
use core::panic;
use ratatui::{
  prelude::*,
  widgets::{block::Block, BorderType, Borders, Paragraph, Wrap},
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

#[derive(Debug)]
pub struct Stats {}

impl Stats {
  pub fn new() -> Stats {
    Stats {}
  }

  pub fn calc_num_rows() -> usize {
    let path = "/home/david/src/github.com/mps/data/netlib/afiro";
    let input = std::fs::read_to_string(path).unwrap();
    match mps::Parser::<f32>::parse(&input) {
      Ok(parsed) => parsed.rows.len(),
      Err(e) => panic!(),
    }
  }

  fn stats() -> Paragraph<'static> {
    let num_rows = Self::calc_num_rows();
    let text = vec![
      Line::from("stats"),
      //
      Line::from(format!("{}", num_rows)),
    ];
    Paragraph::new(text)
      .block(
        Block::default()
          .title("Stats")
          .title_alignment(Alignment::Left)
          .borders(Borders::ALL)
          .border_style(Style::default())
          .border_type(BorderType::Rounded),
      )
      .style(Style::default())
      .wrap(Wrap { trim: false })
      .alignment(Alignment::Left)
  }

  /// Creates a tree widget for displaying statistics.
  ///
  /// # Arguments
  /// - `counters`: The `Counters` containing statistical data.
  ///
  /// # Returns
  /// A `Result` containing the tree widget and its state or an error.
  fn tree() -> Result<(Tree<'static, usize>, TreeState<usize>)> {
    let mut state = TreeState::default();
    let nodes = TreeItem::new(
      1,
      "foobar",
      vec![
        TreeItem::new_leaf(2, format!("Rows: {}", 0)),
        TreeItem::new_leaf(3, format!("Columns: {}", 0)),
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

  fn layer(area: Rect) -> Rect {
    area
  }

  pub fn render(&self, area: Rect, f: &mut Frame<'_>) -> Result<()> {
    let layer = Self::layer(area);
    let (tree, mut state) = Self::tree()?;
    f.render_stateful_widget(tree, layer, &mut state);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_region_new() {
    let _ = Stats::new();
  }
}
