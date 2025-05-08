use std::{
  cell::{Ref, RefCell},
  collections::{HashMap, VecDeque},
  fs::DirEntry,
  ops::Deref,
  path::PathBuf,
  rc::Rc,
  time::Duration,
};

use anyhow::Result;
use crossterm::{ExecutableCommand, event, terminal};
use futures::{FutureExt, StreamExt, select};
use tracing::error;

use zhazba_action::{Action, KeyAction};
use zhazba_buffer::{Buffer, BufferInner, BufferManager};
use zhazba_config::Config;
use zhazba_plugin::Plugin;
use zhazba_render::TermRender;


#[derive(Clone, Debug)]
pub struct Editor(Rc<RefCell<EditorInner>>);
impl Editor {
  pub(crate) const DEFAULT_MODE: char = 'n';
  pub(crate) const BUFFER_MODE: char = 'i';
  pub(crate) const COMMAND_REGISTER: &str = "cmd";


  pub fn new(
    workspace: Option<PathBuf>,
    render: TermRender,
    size: (u16, u16),
  ) -> Self {
    return Self(Rc::new(RefCell::new(EditorInner::new(
      workspace, render, size,
    ))));
  }
}
impl Deref for Editor {
  type Target = Rc<RefCell<EditorInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}

#[derive(Clone, Debug)]
pub struct EditorInner {
  pub(crate) config: Config,
  pub(crate) buffer_manager: BufferManager,
  workspace: Option<PathBuf>,

  pub(crate) mode: char,
  pub(crate) render: TermRender,

  plugin: Plugin,

  size: (u16, u16),
  pub(crate) pos: (usize, usize),
  v_pos: (usize, usize),

  pub(crate) register_map: HashMap<Rc<str>, String>,
  pub(crate) current_register: Option<Rc<str>>,

  pub(crate) actions_queqe: VecDeque<Action>,
}
impl EditorInner {
  pub fn new(
    workspace: Option<PathBuf>,
    render: TermRender,
    size: (u16, u16),
  ) -> Self {
    let register_map =
      HashMap::from_iter([(Rc::from(Editor::COMMAND_REGISTER), String::new())]);

    let mut editor = Self {
      config: Config::default(),

      workspace,
      buffer_manager: BufferManager::new(),
      mode: Editor::DEFAULT_MODE,

      plugin: Plugin::new(),

      render,
      size,
      pos: (0, 0),
      v_pos: (0, 0),

      register_map,
      current_register: None,

      actions_queqe: VecDeque::new(),
    };
    let _ = editor.load_dir();


    return editor;
  }
  fn load_dir(&mut self) -> Result<()> {
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


    return Ok(());
    fn visit_dirs(dir: &PathBuf, cb: &mut dyn FnMut(&DirEntry)) -> Result<()> {
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
  pub(crate) fn keymaps(&self) -> Ref<'_, HashMap<(String, char), KeyAction>> {
    return Ref::map(self.config.borrow(), |config| return &config.keymaps);
  }
  pub(crate) fn commands(&self) -> Ref<'_, HashMap<String, KeyAction>> {
    return Ref::map(self.config.borrow(), |config| return &config.commands);
  }


  pub async fn run(&mut self) -> Result<()> {
    self.plugin.borrow_mut().init()?;

    let mut event_stream = event::EventStream::new();

    self.render.borrow().draw_frame()?;
    loop {
      let mut delay =
        futures_timer::Delay::new(Duration::from_millis(100)).fuse();
      let mut event = event_stream.next().fuse();

      select! {
        _ = delay => {
        }

        event = event => {
          match event {
            Some(Ok(event)) => {
              if self.is_cmd_register() {
                if let Some(ka) = self.handle_command_event(&event) {
                  self.handle_key_action(ka);
                  if self.execute_actions()? {
                    break;
                  };
                };

                continue;
              };

              if let Some(ka) = self.handle_event(&event) {
                self.handle_key_action(ka);
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
      };
    }

    terminal::disable_raw_mode()?;
    self
      .render
      .borrow_mut()
      .stdout
      .borrow_mut()
      .backend_mut()
      .execute(terminal::LeaveAlternateScreen)?;
    return Ok(());
  }
}
