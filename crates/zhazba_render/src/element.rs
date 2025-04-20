use std::io::{self, Write};

use anyhow::Result;
use crossterm::{
  QueueableCommand, cursor,
  style::{self, ContentStyle, StyledContent},
};
use ropey::Rope;

use crate::{Drawable, GUIDrawable, TermDrawable};


#[derive(Debug)]
pub struct Box {
  pos: (u16, u16),
  size: (u16, u16),

  z_idx: i32,

  content: Rope,
}
impl TermDrawable for Box {
  fn t_draw(&mut self) -> Result<()> {
    let mut stdout = io::stdout();

    for (idx, ch) in self.content.chars().enumerate() {
      let idx = idx as u16;
      let x = self.pos.0 + idx % self.size.0;
      let y = self.pos.1 + idx / self.size.0;
      if x > self.pos.0 + self.size.0 || y > self.pos.1 + self.size.1 {
        continue;
      };


      // TODO: style stuff
      stdout
        .queue(cursor::MoveTo(x, y))?
        .queue(style::PrintStyledContent(StyledContent::new(
          ContentStyle::new(),
          ch,
        )))?;
    }

    stdout.flush()?;
    return Ok(());
  }
}
impl GUIDrawable for Box {}
impl Drawable for Box {}
