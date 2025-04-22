use std::{
  cell::RefCell,
  io::{Stdout, stdout},
  ops::Deref,
  rc::Rc,
};

use anyhow::Result;
use crossterm::{ExecutableCommand, QueueableCommand, cursor, terminal};
use ratatui::{Terminal, prelude::CrosstermBackend};


pub fn terminal_size() -> Result<(u16, u16)> {
  return Ok(terminal::size()?);
}

pub trait Render {}
pub struct Renderer<T>(T)
where
  T: Render;

#[derive(Debug, Clone)]
pub struct TermRender(Rc<RefCell<RenderInner>>);
impl TermRender {
  pub fn new() -> Result<Self> {
    let mut stdout = stdout();
    stdout
      .execute(terminal::EnterAlternateScreen)?
      .execute(terminal::Clear(terminal::ClearType::All))?;
    let stdout = Terminal::new(CrosstermBackend::new(stdout))?;
    let stdout = Rc::new(RefCell::new(stdout));

    return Ok(Self(Rc::new(RefCell::new(RenderInner { stdout }))));
  }
}
impl Deref for TermRender {
  type Target = Rc<RefCell<RenderInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}

#[derive(Debug, Clone)]
pub struct RenderInner {
  stdout: Rc<RefCell<Terminal<CrosstermBackend<Stdout>>>>,
}
impl RenderInner {
  pub fn draw_frame(&self) -> Result<()> {
    self.stdout.borrow_mut().flush()?;
    return Ok(());
  }

  pub fn draw_cursor(&self, x: u16, y: u16) -> Result<()> {
    self
      .stdout
      .borrow_mut()
      .backend_mut()
      .queue(cursor::MoveTo(x, y))?;

    return Ok(());
  }
}
