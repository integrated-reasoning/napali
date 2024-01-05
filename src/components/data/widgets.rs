use ratatui::{
  prelude::*,
  widgets::{block::Block, BorderType, Borders},
};

#[derive(Debug)]
pub struct Region<'a> {
  pub block: Block<'a>,
}

impl<'a> Region<'a> {
  pub fn new() -> Region<'a> {
    Region {
      block: Block::default()
        .title("Region")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_style(Style::default())
        .border_type(BorderType::Rounded),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_region_new() {
    let _ = Region::new();
  }
}
