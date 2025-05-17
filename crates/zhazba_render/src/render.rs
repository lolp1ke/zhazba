use std::{
  fmt::Debug,
  io::{Stdout, Write, stdout},
  ops::Deref,
  sync::Arc,
};

use anyhow::Result;
use crossterm::{ExecutableCommand, QueueableCommand, cursor, terminal};
use parking_lot::RwLock;
use ratatui::{
  Frame, Terminal,
  layout::{Constraint, Direction, Layout, Rect},
  prelude::CrosstermBackend,
  style::Styled,
  widgets::{Block, Paragraph, Tabs},
};
use tracing::{error, info};

use zhazba_buffer::Buffer;


#[derive(Clone, Debug)]
pub struct UiNode(Arc<RwLock<UiNodeInner>>);
impl UiNode {
  pub fn new(inner: UiNodeInner) -> Self {
    return Self(Arc::new(RwLock::new(inner)));
  }
  pub fn raw(raw: Arc<RwLock<UiNodeInner>>) -> Self {
    return Self(raw);
  }
}
impl Deref for UiNode {
  type Target = Arc<RwLock<UiNodeInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}

#[derive(Debug)]
pub enum UiNodeInner {
  Block {
    widget: Block<'static>,
    direction: Direction,
    children: Vec<(UiNode, Constraint)>,
  },
  Tabs {
    widget: Tabs<'static>,
    children: Vec<(UiNode, Constraint)>,
  },

  Paragraph {
    widget: Paragraph<'static>,
  },

  // maybe add rect for positioning
  Buffer(Arc<Buffer>),
}
impl UiNodeInner {
  fn render(&self, frame: &mut Frame, area: Rect) {
    match &self {
      Self::Block {
        widget,
        direction,
        children,
      } => {
        let layout = Layout::default()
          .direction(direction.clone())
          .constraints(children.iter().map(|(_, constraint)| constraint))
          .split(area);
        for ((child, _), &area) in children.iter().zip(layout.iter()) {
          child.read_arc().render(frame, area);
        }

        frame.render_widget(widget, area);
      }
      Self::Paragraph { widget } => {
        frame.render_widget(widget, area);
      }
      Self::Buffer(buffer) => {
        let buffer = buffer.read_arc();
        frame.render_widget(Paragraph::new(buffer.as_str()), area);
      }

      _ => error!("Render method not implemented for: {:?}", self),
    };
  }

  pub fn append_child(&mut self, node: UiNode, constraint: Constraint) {
    match self {
      Self::Block { children, .. } | Self::Tabs { children, .. } => {
        children.push((node, constraint));
      }

      _ => {}
    };
  }

  pub fn text(&mut self, text: String) {
    match self {
      Self::Paragraph { widget } => {
        *self = Self::Paragraph {
          widget: Paragraph::new(text).set_style(Styled::style(widget)),
        };
      }

      _ => info!("change not applied"),
    };
  }
}

#[derive(Clone, Debug)]
pub struct TermRender(Arc<RwLock<TermRenderInner>>);
impl TermRender {
  pub fn new() -> Result<Self> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    stdout
      .execute(terminal::EnterAlternateScreen)?
      .execute(terminal::Clear(terminal::ClearType::All))?;
    let stdout = Terminal::new(CrosstermBackend::new(stdout))?;


    let mut node = UiNodeInner::Block {
      widget: Block::new(),
      direction: Direction::Vertical,
      children: Vec::new(),
    };
    node.append_child(
      UiNode::new(UiNodeInner::Block {
        widget: Block::default(),
        direction: Direction::Horizontal,
        children: Vec::new(),
      }),
      Constraint::Percentage(100),
    );
    let node = UiNode(Arc::new(RwLock::new(node)));


    return Ok(Self(Arc::new(RwLock::new(TermRenderInner {
      stdout,

      node,
    }))));
  }
  pub fn cleanup() -> Result<()> {
    terminal::disable_raw_mode()?;
    stdout()
      .queue(terminal::LeaveAlternateScreen)?
      .queue(cursor::MoveTo(0, 0))?
      .queue(cursor::EnableBlinking)?
      .flush()?;


    return Ok(());
  }
}
impl Deref for TermRender {
  type Target = Arc<RwLock<TermRenderInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}


#[derive(Debug)]
pub struct TermRenderInner {
  pub stdout: Terminal<CrosstermBackend<Stdout>>,

  pub node: UiNode,
}
impl TermRenderInner {
  pub fn draw_frame(&mut self) -> Result<()> {
    self.stdout.draw(|frame| {
      self.node.write_arc().render(frame, frame.area());
    })?;

    return Ok(());
  }
}


#[cfg(test)]
mod test {
  use ratatui::{
    layout::{Constraint, Direction},
    widgets::{Block, Paragraph},
  };
  use tracing::info;

  use super::{UiNode, UiNodeInner};

  #[test]
  fn a() {
    let mut window = UiNode::new(UiNodeInner::Block {
      widget: Block::new(),
      direction: Direction::Horizontal,
      children: Vec::new(),
    });
    let node = UiNode::new(UiNodeInner::Paragraph {
      widget: Paragraph::new("mode"),
    });
    window
      .write_arc()
      .append_child(node.clone(), Constraint::Min(1));

    println!("{:?}", node.read_arc());


    node.write_arc().text("text".to_string());
    println!("{:?}", node.read_arc());
    println!("window: {:?}", window);
    node.write_arc().text("other".to_string());
    println!("window: {:?}", window);
  }
}
