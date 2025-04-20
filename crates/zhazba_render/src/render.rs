use std::fmt::Debug;

use anyhow::Result;


pub trait Render: Debug {
  fn draw_frame(&mut self) {}
}


pub trait Drawable: TermDrawable + GUIDrawable {
  fn draw(&mut self) -> Result<()> {
    self.t_draw()?;

    return Ok(());
  }
}
pub trait TermDrawable {
  fn t_draw(&mut self) -> Result<()>;
}
pub trait GUIDrawable {}
