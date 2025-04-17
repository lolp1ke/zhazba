use std::io;

use anyhow::Result;
use crossterm::{ExecutableCommand, terminal};

use crate::Render;


pub fn terminal_size() -> Result<(u16, u16)> {
  return Ok(terminal::size()?);
}

#[derive(Debug)]
pub struct TermRender {
  v_pos: (usize, usize),

  stdout: io::Stdout,
}
impl TermRender {
  pub fn new() -> Result<Self> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout
      .execute(terminal::EnterAlternateScreen)?
      .execute(terminal::Clear(terminal::ClearType::All))?;


    return Ok(Self {
      v_pos: (0, 0),
      stdout,
    });
  }
}
impl Drop for TermRender {
  fn drop(&mut self) {
    let _ = terminal::disable_raw_mode();
    let _ = self.stdout.execute(terminal::LeaveAlternateScreen);
  }
}
impl Render for TermRender {
  fn draw_frame(&mut self, content: String) {}
}
