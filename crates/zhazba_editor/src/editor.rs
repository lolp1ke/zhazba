use std::{
  cell::{RefCell, RefMut},
  collections::VecDeque,
  fs::DirEntry,
  ops::Deref,
  path::PathBuf,
  rc::Rc,
  time,
};

use anyhow::Result;

use crossterm::event;
use futures::{FutureExt, StreamExt, select};
use tracing::{error, info};
use zhazba_action::{Action, KeyAction};
use zhazba_buffer::{Buffer, BufferInner, BufferManager};
use zhazba_config::Config;
use zhazba_lua::{lua_method, lua_userdata};
use zhazba_render::TermRender;


#[derive(Debug, Clone)]
pub struct Editor(Rc<RefCell<EditorInner>>);
#[lua_userdata]
impl Editor {
  pub fn new(
    workspace: Option<PathBuf>,
    render: TermRender,
    size: (u16, u16),
  ) -> Self {
    return Self(Rc::new(RefCell::new(EditorInner::new(
      workspace, render, size,
    ))));
  }

  #[lua_method]
  fn config(&self) -> Config {
    return self.borrow().config.clone();
  }
}
impl Deref for Editor {
  type Target = Rc<RefCell<EditorInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}

#[derive(Debug, Clone)]
pub struct EditorInner {
  pub config: Config,
  workspace: Option<PathBuf>,
  buffer_manager: BufferManager,
  // buffers: VecDeque<RefCell<BufferInner>>,
  // buffer_idx: usize,
  mode: char,

  render: TermRender,
  // render: Rc<RefCell<Box<dyn Render>>>,
  size: (u16, u16),
  pos: (usize, usize),
  v_pos: (usize, usize),

  actions_queqe: VecDeque<Action>,
}
impl EditorInner {
  pub fn new(
    workspace: Option<PathBuf>,
    render: TermRender,
    size: (u16, u16),
  ) -> Self {
    return Self {
      config: Config::default(),

      workspace,
      buffer_manager: BufferManager::new(),
      // buffers: VecDeque::new(),
      // buffer_idx: usize::MAX,
      mode: 'n',

      render,
      // render: Rc::new(RefCell::new(Box::new(render))),
      size,
      pos: (0, 0),
      v_pos: (0, 0),

      actions_queqe: VecDeque::new(),
    };
  }
  pub fn load_dir(&mut self) -> anyhow::Result<()> {
    if let Some(workspace) = self.workspace.as_ref() {
      visit_dirs(workspace, &mut |dir_entry: &DirEntry| {
        let buffer: BufferInner = BufferInner::load_from_file(dir_entry.path());
        let buffer: Buffer = Buffer::new(buffer);
        self.buffer_manager.push_front(buffer);
      })?;
    };

    info!("Buffers: {:#?}", *self.buffer_manager);
    return Ok(());


    fn visit_dirs(
      dir: &PathBuf,
      cb: &mut dyn FnMut(&DirEntry),
    ) -> anyhow::Result<()> {
      if !dir.is_dir() {
        return Ok(());
      };
      for entry in std::fs::read_dir(dir)? {
        let entry: DirEntry = entry?;
        let path: PathBuf = entry.path();
        if path.is_dir() {
          visit_dirs(&path, cb)?;
        } else {
          cb(&entry);
        };
      }

      return Ok(());
    }
  }


  // fn render(&self) -> RefMut<'_, Box<dyn Render>> {
  // return self.render.borrow_mut();
  // }

  fn handle_event(&self, event: event::Event) -> Option<KeyAction> {
    match event {
      event::Event::Key(event::KeyEvent {
        code,
        modifiers,
        kind: event::KeyEventKind::Press,
        ..
      }) => {
        let key_code = format!(
          "{}{}{}",
          modifiers,
          if modifiers.is_empty() { "" } else { "-" },
          code
        )
        .to_lowercase();

        return self
          .config
          .borrow()
          .keymaps
          .get(&(key_code, self.mode))
          .cloned();
      }

      _ => return None,
    };
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

        return Ok(true);
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
                info!("Key action: {:?}", key_action);
                self.handle_key_action(key_action);
              };
            }

            Some(Err(err)) => {
              error!("{err}");
            }
            None => {}
          };

          // self.render().draw_frame()?;
          if self.execute_actions()? {
            break;
          };
        }
      }
    }

    return Ok(());
  }
}
