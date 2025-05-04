use std::{
  cell::RefCell,
  collections::{HashMap, VecDeque},
  fs::DirEntry,
  ops::Deref,
  path::PathBuf,
  rc::Rc,
  time,
};

use anyhow::Result;
use crossterm::{ExecutableCommand, event, terminal};
use futures::{FutureExt, StreamExt, select};
use tracing::{error, info};

use zhazba_action::{Action, KeyAction};
use zhazba_buffer::{Buffer, BufferInner, BufferManager};
use zhazba_config::Config;
use zhazba_lua::{lua_method, lua_userdata};
use zhazba_plugin::Plugin;
use zhazba_render::TermRender;


#[derive(Debug, Clone)]
pub struct Editor(Rc<RefCell<EditorInner>>);
#[lua_userdata]
impl Editor {
  const DEFAULT_MODE: char = 'n';
  const BUFFER_MODE: char = 'i';


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
  #[lua_method]
  fn render(&self) -> TermRender {
    return self.borrow().render.clone();
  }
  #[lua_method]
  fn create_register(&self, mode: String) {
    self
      .borrow_mut()
      .registers
      .insert(mode.chars().next().unwrap(), String::new());
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
  config: Config,
  pub(crate) buffer_manager: BufferManager,
  workspace: Option<PathBuf>,

  mode: char,
  render: TermRender,

  plugin: Plugin,

  size: (u16, u16),
  pos: (usize, usize),
  v_pos: (usize, usize),

  registers: HashMap<char, String>,

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
      mode: Editor::DEFAULT_MODE,

      plugin: Plugin::new(),

      render,
      size,
      pos: (0, 0),
      v_pos: (0, 0),

      registers: HashMap::new(),

      actions_queqe: VecDeque::new(),
    };
  }
  pub fn load_dir(&mut self) -> anyhow::Result<()> {
    if let Some(workspace) = self.workspace.as_ref() {
      if workspace.is_file() {
        self.buffer_manager.push_front(Buffer::new(
          BufferInner::load_from_file(workspace.clone()),
        ));

        return Ok(());
      };

      visit_dirs(workspace, &mut |dir_entry: &DirEntry| {
        let buffer: BufferInner = BufferInner::load_from_file(dir_entry.path());
        let buffer: Buffer = Buffer::new(buffer);
        self.buffer_manager.push_front(buffer);
      })?;
    };

    // info!("Buffers: {:#?}", *self.buffer_manager);
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

        let ka = self
          .config
          .borrow()
          .keymaps
          .get(&(key_code, self.mode))
          .cloned();

        if let None = ka {
          if self.mode == Editor::BUFFER_MODE {
            return Some(KeyAction::Multiple(vec![
              Action::InsertIntoBufferAt(
                self.pos.0,
                self.pos.1,
                code.to_string(),
              ),
              Action::MoveRight,
            ]));
          };

          for (&register_mode, _) in self.registers.iter() {
            if register_mode == self.mode {
              return Some(KeyAction::Single(Action::InsertIntoRegister(
                format!("{}", register_mode),
                format!("{}", code),
              )));
            };
          }
        };

        return ka;
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
  fn execute_action(&mut self, action: Action) -> Result<bool> {
    use Action::*;

    match action {
      Quit(force) => {
        if force {
          return Ok(true);
        };

        return Ok(true);
      }
      // Save => {}
      ChangeMode(mode) => {
        self.mode = mode.chars().next().unwrap_or_else(|| Editor::DEFAULT_MODE);
      }

      MoveLeft => todo!(),
      MoveRight => self.pos.0 += 1,
      MoveUp => todo!(),
      MoveDown => todo!(),

      InsertIntoRegister(mode, append_char) => {
        if let Some(content) = self
          .registers
          .get_mut(&mode.chars().next().unwrap_or_else(|| '\0'))
        {
          content.push_str(&append_char);
        };
      }
      InsertIntoBufferAt(cx, cy, append_char) => {
        self.insert_into_buffer((cx, cy), &append_char);
      }

      _ => error!("Action: {:?} is not implemented yet", action),
    };

    return Ok(false);
  }
  fn cursor_pos(&self) -> (u16, u16) {
    return (self.pos.0 as u16, self.pos.1 as u16);
  }

  pub async fn run(&mut self) -> Result<()> {
    self.plugin.borrow_mut().init()?;

    let mut event_stream = event::EventStream::new();

    self.render.borrow().draw_frame()?;
    loop {
      let mut delay =
        futures_timer::Delay::new(time::Duration::from_millis(100)).fuse();
      let mut event = event_stream.next().fuse();

      select! {
        _ = delay => {
        }

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


          self.render.borrow_mut().draw_frame()?;
          if self.execute_actions()? {
            break;
          };
        }
      }
    }

    terminal::disable_raw_mode()?;
    self
      .render
      .borrow_mut()
      .stdout
      .borrow_mut()
      .backend_mut()
      .execute(terminal::LeaveAlternateScreen)?;

    info!("{:?}", self.buffer_manager.deref()[0]);
    return Ok(());
  }
}
