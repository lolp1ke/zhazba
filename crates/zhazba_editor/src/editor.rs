use std::{
  cell::{RefCell, RefMut},
  collections::VecDeque,
  path::PathBuf,
  time,
};

use anyhow::Result;

use crossterm::event;
use futures::{FutureExt, StreamExt, select};
use tracing::error;
use zhazba_action::{Action, KeyAction};
use zhazba_buffer::Buffer;
use zhazba_config::Config;
use zhazba_render::Render;


#[derive(Debug)]
pub struct Editor {
  config: Config,

  workspace: Option<PathBuf>,
  buffers: VecDeque<RefCell<Buffer>>,
  buffer_idx: usize,

  mode: char,

  render: RefCell<Box<dyn Render>>,
  size: (u16, u16),
  pos: (usize, usize),
  v_pos: (usize, usize),

  actions_queqe: VecDeque<Action>,
}
impl Editor {
  pub fn new(
    workspace: Option<PathBuf>,
    render: impl Render + 'static,
    size: (u16, u16),
  ) -> Self {
    return Self {
      config: Config::default(),

      workspace,
      buffers: VecDeque::new(),
      buffer_idx: usize::MAX,

      mode: '\0',

      render: RefCell::new(Box::new(render)),
      size,
      pos: (0, 0),
      v_pos: (0, 0),

      actions_queqe: VecDeque::new(),
    };
  }

  fn render(&self) -> RefMut<'_, Box<dyn Render>> {
    return self.render.borrow_mut();
  }

  fn handle_event(&self, event: event::Event) -> Option<KeyAction> {
    match event {
      event::Event::Key(event::KeyEvent {
        code,
        modifiers,
        kind: event::KeyEventKind::Press,
        ..
      }) => {
        let key_code = format!("{}-{}", modifiers, code);
      }

      _ => return None,
    };

    todo!()
  }
  fn handle_key_action(&mut self, key_action: KeyAction) {
    use KeyAction::*;

    match key_action {
      Single(action) => {
        self.actions_queqe.push_back(action);
      }
      Multiple(actions) => {
        for action in actions {
          self.actions_queqe.push_back(action);
        }
      }
      Nested(keymap) => {
        todo!("{:?}", keymap);
      }
    };
  }
  fn execute_actions(&mut self) -> Result<bool> {
    let mut quit = false;
    while let Some(action) = self.actions_queqe.pop_front() {
      quit = self.execute_action(action)?;
    }

    return Ok(quit);
  }
  fn execute_action(&self, action: Action) -> Result<bool> {
    use Action::*;

    match action {
      Quit(force) => {
        if force {
          return Ok(true);
        };
      }

      _ => error!("Action: {:?} is not implemented yet", action),
    };

    return Ok(false);
  }

  pub async fn run(&mut self) -> Result<()> {
    let mut event_stream = event::EventStream::new();
    loop {
      let mut delay =
        futures_timer::Delay::new(time::Duration::from_millis(100)).fuse();
      let mut event = event_stream.next().fuse();

      select! {
        _ = delay => {}

        event = event => {
          match event {
            Some(Ok(event)) => {
              if let Some(key_action) = self.handle_event(event) {
                self.handle_key_action(key_action);
              };
            }

            Some(Err(err)) => {
              error!("{err}");
            }
            None => {}
          };

          // self.render().draw_frame();
          if self.execute_actions()? {
            break;
          };
        }
      }
    }

    return Ok(());
  }
}
