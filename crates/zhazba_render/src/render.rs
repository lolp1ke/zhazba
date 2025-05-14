use std::{
  fmt::Debug,
  io::{Stdout, stdout},
  ops::Deref,
  sync::Arc,
};

use anyhow::Result;
use crossterm::{ExecutableCommand, terminal};
use parking_lot::RwLock;
use ratatui::{
  Frame, Terminal,
  layout::{Constraint, Direction, Layout, Rect},
  prelude::CrosstermBackend,
  widgets::{Block, Paragraph, Tabs},
};
use tracing::error;

use zhazba_buffer::Buffer;
use zhazba_lua::lua_userdata;


pub fn terminal_size() -> Result<(u16, u16)> {
  return Ok(terminal::size()?);
}
pub fn disable_raw_mode() -> Result<()> {
  return Ok(terminal::disable_raw_mode()?);
}

#[derive(Clone, Debug)]
pub enum UiNode {
  Block {
    widget: Block<'static>,
    direction: Direction,
    children: Vec<Self>,
  },
  Tabs {
    widget: Tabs<'static>,
    children: Vec<Self>,
  },

  Paragraph {
    widget: Paragraph<'static>,
  },

  // maybe add rect for positioning
  Buffer(Arc<Buffer>),
}
impl UiNode {
  fn render(&self, frame: &mut Frame, area: Rect) {
    match &self {
      Self::Block {
        widget,
        direction,
        children,
      } => {
        let layout = Layout::default()
          .direction(direction.clone())
          .constraints(children.iter().map(|_| Constraint::Fill(1)))
          .split(area);
        for (child, &area) in children.iter().zip(layout.iter()) {
          child.render(frame, area);
        }

        frame.render_widget(widget, area);
      }
      Self::Paragraph { widget } => {
        frame.render_widget(widget, area);
      }
      Self::Buffer(buffer) => {
        let buffer = buffer.read();
        // let lines = buffer.lines();
        frame.render_widget(Paragraph::new(buffer.as_str()), area);

        // for line in lines {
        // frame.render_widget(, area);
        // }
      }

      _ => error!("Render method not implemented for: {:?}", self),
    };
  }

  // fn load_buffer(&mut self, content: String) {
  //   match self {
  //     Self::Paragraph { widget } => {
  //       *widget = Paragraph::new(content);
  //     }
  //     _ => {}
  //   };
  // }
  pub fn append_child(&mut self, node: Self) {
    match self {
      Self::Block { children, .. } | Self::Tabs { children, .. } => {
        children.push(node)
      }

      _ => {}
    };
  }
}

#[derive(Clone, Debug)]
pub struct TermRender(Arc<RwLock<TermRenderInner>>);
#[lua_userdata]
impl TermRender {
  pub fn new() -> Result<Self> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    stdout
      .execute(terminal::EnterAlternateScreen)?
      .execute(terminal::Clear(terminal::ClearType::All))?;
    let stdout = Terminal::new(CrosstermBackend::new(stdout))?;
    let stdout = Arc::new(RwLock::new(stdout));


    let node = UiNode::Block {
      widget: Block::new(),
      direction: Direction::Vertical,
      children: Vec::new(),
    };

    let layout =
      Layout::new(Direction::Horizontal, [Constraint::Percentage(100)]);

    return Ok(Self(Arc::new(RwLock::new(TermRenderInner {
      stdout,

      node,
      layout,
    }))));
  }

  // #[lua_method]
  // fn window(&self) -> UiNode {
  // return self.borrow().node.clone();
  // }
}
impl Deref for TermRender {
  type Target = Arc<RwLock<TermRenderInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}


#[derive(Debug)]
pub struct TermRenderInner {
  pub stdout: Arc<RwLock<Terminal<CrosstermBackend<Stdout>>>>,

  pub node: UiNode,
  layout: Layout,
}
impl TermRenderInner {
  pub fn draw_frame(&self) -> Result<()> {
    self.stdout.write_arc().draw(|frame| {
      self.node.render(frame, frame.area());
      // let chunks = self.layout.split(frame.area());

      // for ((_, node), &area) in self.node.iter().zip(chunks.iter()) {
      // node.render(frame, area);
      // }
    })?;

    return Ok(());
  }

  fn draw_cursor(&self, x: u16, y: u16) -> Result<()> {
    // self
    // .stdout
    // .borrow_mut()
    // .backend_mut()
    // .queue(cursor::MoveTo(x, y))?;

    return Ok(());
  }
}
