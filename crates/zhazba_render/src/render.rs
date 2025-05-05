use std::{
  cell::RefCell,
  fmt::Debug,
  io::{Stdout, stdout},
  ops::Deref,
  rc::Rc,
};

use anyhow::Result;
use crossterm::{ExecutableCommand, QueueableCommand, cursor, terminal};
use ratatui::{
  Frame, Terminal,
  layout::{Constraint, Direction, Layout, Rect},
  prelude::CrosstermBackend,
  widgets::{Block, Paragraph, Tabs},
};

use zhazba_buffer::Buffer;
use zhazba_lua::{lua_method, lua_userdata};


pub fn terminal_size() -> Result<(u16, u16)> {
  return Ok(terminal::size()?);
}
pub fn disable_raw_mode() -> Result<()> {
  terminal::disable_raw_mode()?;
  return Ok(());
}


// // #[ui_nodes]
// #[derive(Clone, Debug)]
// pub enum UiNode {
//   Block(Block<'static>, HashMap<String, Self>),
//   Tabs(Tabs<'static>, Vec<Self>),
//   Paragraph(Paragraph<'static>),
// }
// impl UiNode {
//   fn render(&self, frame: &mut Frame, area: Rect) {
//     match self.clone() {
//       Self::Block(block, children) => {
//         let inner = block.inner(area);
//         frame.render_widget(block, area);
//         let chunks = Layout::default()
//           .direction(Direction::Vertical)
//           .constraints(
//             children
//               .iter()
//               .map(|_| Constraint::Fill(1))
//               .collect::<Vec<_>>(),
//           )
//           .split(inner);

//         for ((_, child), &area) in children.iter().zip(chunks.iter()) {
//           child.render(frame, area);
//         }
//       }
//       Self::Paragraph(paragraph) => {
//         frame.render_widget(paragraph, area);
//       }
//       Self::Tabs(tabs, children) => {
//         frame.render_widget(tabs, area);

//         for child in children.iter() {
//           child.render(frame, area);
//         }
//       }

//       _ => todo!(),
//     };
//   }


//   pub(crate) fn make_block() -> Self {
//     return Self::Block(Block::default(), HashMap::default());
//   }
// }

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
  Buffer(Rc<Buffer>),
}
impl UiNode {
  fn render(&self, frame: &mut Frame, area: Rect) {}

  fn load_buffer(&mut self, content: String) {
    match self {
      Self::Paragraph { widget } => {
        *widget = Paragraph::new(content);
      }
      _ => {}
    };
  }
  fn append_child(&mut self, node: Self) {
    match self {
      Self::Block { children, .. } | Self::Tabs { children, .. } => {
        children.push(node)
      }

      _ => {}
    };
  }
}

#[derive(Clone, Debug)]
pub struct TermRender(Rc<RefCell<TermRenderInner>>);
#[lua_userdata]
impl TermRender {
  pub fn new() -> Result<Self> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    stdout
      .execute(terminal::EnterAlternateScreen)?
      .execute(terminal::Clear(terminal::ClearType::All))?;
    let stdout = Terminal::new(CrosstermBackend::new(stdout))?;
    let stdout = Rc::new(RefCell::new(stdout));


    let mut node = UiNode::Block {
      widget: Block::new(),
      direction: Direction::Vertical,
      children: Vec::new(),
    };
    node.append_child(UiNode::Paragraph {
      widget: Paragraph::default(),
    });

    let layout =
      Layout::new(Direction::Horizontal, [Constraint::Percentage(100)]);

    return Ok(Self(Rc::new(RefCell::new(TermRenderInner {
      stdout,

      node,
      layout,
    }))));
  }

  #[lua_method]
  fn window(&self) -> UiNode {
    return self.borrow().node.clone();
  }
}
impl Deref for TermRender {
  type Target = Rc<RefCell<TermRenderInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}


#[derive(Debug)]
pub struct TermRenderInner {
  pub stdout: Rc<RefCell<Terminal<CrosstermBackend<Stdout>>>>,

  node: UiNode,
  layout: Layout,
}
impl TermRenderInner {
  pub fn draw_frame(&self) -> Result<()> {
    // self.stdout.borrow_mut().draw(|frame| {
    //   let chunks = self.layout.split(frame.area());

    //   for ((_, node), &area) in self.node.iter().zip(chunks.iter()) {
    //     node.render(frame, area);
    //   }
    // })?;

    return Ok(());
  }

  fn draw_cursor(&self, x: u16, y: u16) -> Result<()> {
    self
      .stdout
      .borrow_mut()
      .backend_mut()
      .queue(cursor::MoveTo(x, y))?;

    return Ok(());
  }
}
